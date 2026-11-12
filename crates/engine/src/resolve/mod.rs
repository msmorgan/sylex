//! Resolution ([CR#608]): dispatch a stack object, and walk its `Instruction`
//! AST as reified agenda work. Stage 3 wires the corpus's arms; the rest are
//! `todo!`.

use std::sync::Arc;

use sylex_core::Ability;
use sylex_core::Agency;
use sylex_core::TargetSpec;
use sylex_core::Zone;

use crate::agenda::WorkItem;
use crate::event::AbilityCountered;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::event::ZoneChange;
use crate::object::ObjectId;
use crate::stack::ExecutionFrame;
use crate::stack::StackEntry;
use crate::stack::StackObject;
use crate::state::GameState;

mod action;
mod count;
mod effect;
mod player_action;
mod query;
pub(crate) use query::search_is_bare_quantity;
mod targets;
use targets::ResolvingTargets;

// The mill apply re-derives its top-slice group off the stored body facet.
pub(crate) use action::composite_body_group;
// Target-set machinery moved to `targets`; external callers keep the old
// `crate::resolve::…` paths.
pub(crate) use targets::announce_satisfiable;
pub(crate) use targets::announced_prefix_len;
pub(crate) use targets::distinct_siblings;
pub(crate) use targets::slot_count_bounds;
pub(crate) use targets::target_spec_predicate;
pub(crate) use targets::validate_target_set;

impl GameState {
    #[expect(
        clippy::too_many_arguments,
        reason = "resolution region entry carries the complete stack-entry context"
    )]
    fn resolution_region_frame(
        &self,
        region: &sylex_core::Region,
        activation: crate::ActivationId,
        source: ObjectId,
        controller: crate::PlayerId,
        targets: &[Vec<ObjectId>],
        x: Option<sylex_core::Uint>,
        bindings: Option<&crate::trigger::TriggerBindings>,
    ) -> ExecutionFrame {
        if matches!(activation, crate::ActivationId::Stored(_)) {
            return ExecutionFrame {
                activation,
                payment: None,
            };
        }
        let mut frame = self.frame(source, controller);
        if let Some(bindings) = bindings {
            self.frame_set_source_lki(&mut frame, bindings.this.clone());
            self.frame_set_defending_player(&mut frame, bindings.defending_player);
            self.frame_set_event_bindings(
                &mut frame,
                bindings.that_object.clone(),
                bindings.that_player,
                bindings.that_patient.clone(),
            );
            self.frame_set_event_extras(
                &mut frame,
                bindings.event_amount,
                bindings.produced_mana.clone(),
                bindings.crossed,
            );
        }
        self.frame_set_targets(&mut frame, targets);
        self.frame_set_x(&mut frame, x);
        frame.activation = self.enter_region(region, &frame);
        frame
    }

    /// [CR#608]: resolve the committed stack entry whose `id` is `id`. Schedules
    /// the work and the trailing cleanup event.
    ///
    /// Keyed on `StackEntry.id` (not the backing object) so it resolves both
    /// spells and triggered abilities (which have no backing object).
    ///
    /// # Panics
    ///
    /// Panics if no entry has that id — engine invariant, not caller input.
    pub(crate) fn resolve_object(&mut self, id: ObjectId) {
        let entry = self
            .stack
            .iter()
            .find(|e| e.id == id)
            .expect("entry on stack")
            .clone();
        // [CR#400.7j] is scoped to ONE effect: a fresh resolution starts with
        // an empty move record, so a later effect can never find (or follow)
        // an earlier effect's moves.
        self.moved_chain.clear();
        // [CR#603.12]: a reflexive trigger looks back only over events of the
        // resolution that CREATES it, so the window resets each resolution.
        self.resolution_events.clear();
        // The aggregate finalization ledger has the same one-resolution
        // lifetime: no contained action from an earlier stack entry may
        // satisfy a later aggregate's watcher.
        self.resolution_contained_act_commits.clear();
        self.resolution_contained_act_serial = 0;
        match &entry.object {
            StackObject::Spell(spell) => {
                let spell = *spell;
                if entry.copy && self.is_permanent_spell(spell) {
                    // [CR#707.10f]: a resolving permanent-spell copy should
                    // become a token permanent; until that support lands
                    // (follow-up ticket), it vanishes like any other copy
                    // ([CR#707.10a]).
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityResolved(spell),
                    ))]);
                } else if self.is_permanent_spell(spell) {
                    // [CR#608.3b]: a permanent spell with an illegal target
                    // doesn't resolve. A bestowed Aura or mutating creature
                    // spell would instead resolve as a creature spell; the
                    // engine models neither bestow nor mutate, so no such
                    // spell reaches here.
                    let exit = if self.resolving_targets(&entry).any_illegal() {
                        failed_resolution_exit(&entry)
                    } else {
                        self.permanent_spell_entry(&entry, spell)
                    };
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(exit))]);
                } else {
                    let resolving = self.resolving_targets(&entry);
                    if resolving.resolves() {
                        self.resolve_spell_effect(&entry, spell, &resolving);
                    } else {
                        self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                            failed_resolution_exit(&entry),
                        ))]);
                    }
                }
            }
            // [CR#608.2n]: a triggered ability resolves its effect, then vanishes
            // — no zone move, the source untouched. The minted stack id is just
            // discarded when `AbilityResolved` removes the entry.
            StackObject::Triggered {
                source,
                ability,
                created,
                bindings,
            } => {
                // A delayed/reflexive trigger carries its body by value
                // ([CR#603.7,603.12]); a printed one is read by index — its
                // text may have changed under layers, so re-derive it fresh.
                // The index read uses `.get` (never `[]`): a source that left
                // the battlefield and reminted shorter — or
                // whose current face is shorter than the fired
                // index — yields `None`, and the trigger
                // FIZZLES (no-op, vanishes) rather than panicking, mirroring
                // the intervening-if / illegal-target vanish
                // below and `step::trigger`'s graceful `.get`.
                // (Back-face-sourced printed triggers are
                // captured by value at fire time, so they never
                // reach this fallback; this guards the residual gone/short
                // cases.)
                let t = match created {
                    Some(t) => t.as_ref().clone(),
                    None => {
                        if let Some(t) = crate::derive::abilities_of_source(self, *source)
                            .get(*ability)
                            .and_then(Ability::as_triggered)
                        {
                            t.clone()
                        } else {
                            self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                                GameEvent::AbilityResolved(entry.id),
                            ))]);
                            return;
                        }
                    }
                };
                let source_id = bindings
                    .this
                    .as_ref()
                    .map_or(entry.id, |snapshot| snapshot.object);
                let frame = self.resolution_region_frame(
                    &t.effect,
                    entry.activation,
                    source_id,
                    entry.controller,
                    &entry.targets,
                    entry.x,
                    Some(bindings),
                );
                // [CR#702.21b]: a ward-{X} toll's X "is determined at the time
                // the ability resolves, not locked in as the ability triggers".
                // The region was entered when the trigger was PLACED, so the
                // stored activation's announced-X parameter is filled here, at
                // resolution — the one moment [CR#608.2h] allows the
                // information to be determined. The toll's `{X}` and every
                // X occurrences in the body are then the same indexed read.
                if let Some(where_x) = &t.where_x {
                    let x = self.eval_count(where_x, &frame);
                    self.activation_set_x(frame.activation, x);
                }
                // [CR#603.4]: an intervening-if is rechecked as the ability
                // resolves. If it no longer holds, the ability is removed from
                // the stack and does nothing (the rule mirrors the
                // illegal-target fizzle) — schedule only the
                // `AbilityResolved` that discards the
                // entry, never the effect.
                if t.condition
                    .as_ref()
                    .is_some_and(|c| !self.condition_holds(c, &frame))
                {
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityResolved(entry.id),
                    ))]);
                } else {
                    // [CR#608.2b] follows the intervening-if recheck.
                    let resolving = self.resolving_targets(&entry);
                    let items = if resolving.resolves() {
                        let mut items =
                            self.resolving_effect_items(&entry, &t.effect, &frame, &resolving);
                        items.push(WorkItem::Emit(Occurrence::single(
                            GameEvent::AbilityResolved(entry.id),
                        )));
                        items
                    } else {
                        vec![WorkItem::Emit(Occurrence::single(failed_resolution_exit(
                            &entry,
                        )))]
                    };
                    self.schedule_front(items);
                }
            }
            // [CR#602.2a]: an activated ability resolves its carried text,
            // then vanishes like a trigger — no zone move.
            StackObject::Activated {
                source,
                ability,
                bindings,
            } => {
                let resolving = self.resolving_targets(&entry);
                let items = if resolving.resolves() {
                    let frame = self.resolution_region_frame(
                        &ability.effect,
                        entry.activation,
                        *source,
                        entry.controller,
                        &entry.targets,
                        entry.x,
                        Some(bindings),
                    );
                    let mut items =
                        self.resolving_effect_items(&entry, &ability.effect, &frame, &resolving);
                    items.push(WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityResolved(entry.id),
                    )));
                    items
                } else {
                    vec![WorkItem::Emit(Occurrence::single(failed_resolution_exit(
                        &entry,
                    )))]
                };
                self.schedule_front(items);
            }
        }
    }

    /// [CR#608.3]: the event that puts a resolving permanent spell onto the
    /// battlefield.
    fn permanent_spell_entry(&self, entry: &StackEntry, spell: ObjectId) -> GameEvent {
        // Host resolution by entry context (spec §4, [CR#608.3c]): a
        // permanent SPELL that enters attached (the Enchant `AsEnters`)
        // attaches to its resolving spell's CHOSEN TARGET — the Aura's
        // enchant target — not an arbitrary candidate. Carry that host in
        // the `EnterStatus`; `apply_zone_will_change` prefers it over the
        // candidate-set fallback (`.or`). The Aura's enchant target is slot
        // 0's single member.
        let enters = if self.enters_attached_self(self.objects.obj(spell).source)
            && let Some(&host) = entry.targets.first().and_then(|slot| slot.first())
        {
            Some(crate::event::EnterStatus {
                attach_to: Some(host),
                ..crate::event::EnterStatus::default()
            })
        } else {
            None
        };
        GameEvent::ZoneChange(ZoneChange {
            snapshot: None,
            object: spell,
            from: Some(Zone::Stack),
            to: Zone::Battlefield,
            enters,
            position: None,
            face: None,
            cause: None,
        })
    }

    /// An instant or sorcery that resolves: its effect, then its move to its
    /// owner's graveyard ([CR#608.2n]).
    fn resolve_spell_effect(
        &mut self,
        entry: &StackEntry,
        spell: ObjectId,
        resolving: &ResolvingTargets,
    ) {
        let effect = self
            .spell_effect(spell)
            .expect("an instant/sorcery has a Spell ability");
        let frame = self.resolution_region_frame(
            &effect,
            entry.activation,
            spell,
            entry.controller,
            &entry.targets,
            entry.x,
            None,
        );
        let leave = if entry.copy {
            // [CR#707.10a]: a copy leaves the stack by CEASING to exist — no
            // zone move, no card. Same shape as a triggered ability vanishing
            // ([CR#608.2n]).
            GameEvent::AbilityResolved(spell)
        } else {
            GameEvent::ZoneChange(ZoneChange {
                snapshot: None,
                object: spell,
                from: Some(Zone::Stack),
                to: Zone::Graveyard,
                enters: None,
                position: None,
                face: None,
                cause: None,
            })
        };
        let mut items = self.resolving_effect_items(entry, &effect, &frame, resolving);
        items.push(WorkItem::Emit(Occurrence::single(leave)));
        self.schedule_front(items);
    }

    /// The effect work of an entry that resolves [CR#608.2b]: only its legal
    /// targets are affected or read, and each illegal one reads as no object
    /// in `frame`'s activation and in every mode's target scope.
    fn resolving_effect_items(
        &mut self,
        entry: &StackEntry,
        effect: &sylex_core::Region,
        frame: &ExecutionFrame,
        resolving: &ResolvingTargets,
    ) -> Vec<WorkItem> {
        self.activation_restrict_targets(frame.activation, &resolving.slots);
        crate::cast::announced_effect_items(
            self,
            effect,
            frame,
            entry.chosen_modes.as_ref(),
            &resolving.slots,
        )
    }

    /// Follow the resolution-scoped move record ([CR#400.7j]) to the id the
    /// object became: `a→b→c` chases `a` to `c`. Identity on unrecorded ids.
    /// Remint never reuses a live id, so the walk terminates.
    ///
    /// SEAM (engine-delayed-reflexive-triggers): a delayed trigger created
    /// mid-resolution captures bound roles as ids; that registry must
    /// FINALIZE its captured ids via this chase before the resolution ends
    /// (the record clears when the next resolution begins).
    pub(crate) fn chase_moved(&self, id: ObjectId) -> ObjectId {
        let mut cur = id;
        while let Some(&(_, new)) = self.moved_chain.iter().find(|&&(old, _)| old == cur) {
            cur = new;
        }
        cur
    }

    /// The `ObjectSource` that anchors `Ref(This)`/`Ref(You)` in live filter
    /// evaluation for `frame`: the announce-time snapshot's source when the
    /// frame carries bindings (the live object may be gone, [CR#603.10a]),
    /// else the live source object's.
    pub(crate) fn frame_watcher(&self, frame: &ExecutionFrame) -> crate::object::ObjectSource {
        frame
            .source_lki(self)
            .as_ref()
            .map_or_else(|| self.objects.obj(frame.source(self)).source, |s| s.source)
    }

    /// True iff any of the card's printed types is a PERMANENT type
    /// ([CR#608.3] — a spell of this type enters the battlefield on
    /// resolution instead of resolving as a one-shot effect). Reads the
    /// `TypeDef.permanent_type` flag, not an enum name-list. Grizzly Bears →
    /// true; Instant `DealDamage` `AnyTarget` → false. The
    /// battlefield-entry-vs-resolve control-flow fork this feeds
    /// (`resolve.rs:93`) stays hardcoded.
    #[must_use]
    pub(crate) fn is_permanent_spell(&self, id: ObjectId) -> bool {
        crate::derive::face(self.def(id))
            .characteristics
            .types
            .iter()
            .any(|t| t.permanent_type)
    }

    /// The object's `Ability::Spell`, which carries the targets a spell
    /// announces and the instructions it follows, looked up through composite
    /// keywords: Enchant confers the one that gives an Aura spell the target
    /// its enchant ability defines [CR#303.4a,702.5a]. `None` for a spell
    /// that has none (a creature spell).
    #[must_use]
    pub(crate) fn spell_ability(&self, id: ObjectId) -> Option<Arc<sylex_core::SpellAbility>> {
        spell_ability_in(&crate::derive::abilities(self, id))
    }

    /// Returns the effect of the spell's spell ability, cloned. Returns `None`
    /// if there is no spell ability.
    #[must_use]
    pub(crate) fn spell_effect(&self, id: ObjectId) -> Option<sylex_core::Region> {
        let spell = self.spell_ability(id)?;
        let mut effect = spell.effect.clone();
        if effect.params.is_empty() {
            effect.params = sylex_core::announced_region_params(spell.targets.len());
        }
        Some(effect)
    }

    /// The spell's PRINTED ADDITIONAL cost ([CR#118.8,601.2b]) — the cost
    /// block its `SpellAbility` declares, announced and paid with the mana
    /// cost. Empty for a spell with no additional cost.
    #[must_use]
    pub(crate) fn spell_additional_cost(&self, id: ObjectId) -> sylex_core::Cost {
        self.spell_ability(id)
            .map(|spell| spell.cost.clone())
            .unwrap_or_default()
    }

    #[must_use]
    pub(crate) fn spell_targets(&self, id: ObjectId) -> Vec<TargetSpec> {
        self.spell_ability(id)
            .map_or_else(Vec::new, |spell| spell.targets.to_vec())
    }
}

/// The first spell ability in `abilities`, composite keywords spliced
/// (`derive::flatten_composites`).
pub(crate) fn spell_ability_in(abilities: &[Ability]) -> Option<Arc<sylex_core::SpellAbility>> {
    abilities.iter().find_map(|ability| {
        let mut flat = Vec::new();
        crate::derive::flatten_composites(ability, &mut flat);
        flat.into_iter().find_map(|member| match member {
            Ability::Spell(spell) => Some(spell),
            _ => None,
        })
    })
}

/// The event that removes `entry` from the stack when it doesn't resolve
/// [CR#608.2b,608.3b]: a spell card goes to its owner's graveyard, while an
/// ability or a spell copy leaves no card behind ([CR#707.10a]).
fn failed_resolution_exit(entry: &StackEntry) -> GameEvent {
    let cause = Cause::counter(Agency::StateBasedAction, None);
    match &entry.object {
        StackObject::Spell(spell) if !entry.copy => GameEvent::ZoneChange(ZoneChange {
            snapshot: None,
            object: *spell,
            from: Some(Zone::Stack),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: Some(cause),
        }),
        StackObject::Spell(id) => GameEvent::AbilityCountered(AbilityCountered { id: *id, cause }),
        StackObject::Triggered { .. } | StackObject::Activated { .. } => {
            GameEvent::AbilityCountered(AbilityCountered {
                id: entry.id,
                cause,
            })
        }
    }
}

/// One event → `Single`; several → a simultaneous `Batch`.
fn occurrence_of(mut events: Vec<GameEvent>) -> crate::event::Occurrence {
    use crate::event::Occurrence;
    if events.len() == 1 {
        Occurrence::Single(events.pop().expect("len 1"))
    } else {
        Occurrence::Batch(events)
    }
}

#[cfg(test)]
mod fixtures;

#[cfg(test)]
mod tests {
    #![allow(
        clippy::empty_line_after_doc_comments,
        reason = "related behavioral test rationale is intentionally grouped"
    )]

    use std::sync::Arc;

    use sylex_card::Card;
    use sylex_card::CardFace;
    use sylex_card::Characteristics;
    use sylex_core::Type;
    use sylex_core::Zone;

    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::resolve::fixtures::*;
    use crate::stack::StackEntry;
    use crate::stack::StackObject;
    use crate::state::GameState;

    /// `chase_moved` follows the resolution-scoped old→new move record
    /// transitively ([CR#400.7j]) and is the identity on unrecorded ids.
    #[test]
    fn chase_moved_follows_chain_transitively() {
        let (mut state, a, b) = two_permanents_on_field();
        // Unrecorded: identity.
        assert_eq!(state.chase_moved(a), a);
        // a -> b -> a2. The third id MUST come from the SAME state: a fresh
        // GameState mints in the same order, so its ids numerically collide
        // with this state's and a cross-state id would close the chain into a
        // cycle — chase_moved's termination leans on the within-state
        // ids-never-repeat invariant.
        let a2 = state.objects.mint(
            crate::object::ObjectSource::Player(PlayerId(1)),
            PlayerId(1),
            Some(Zone::Battlefield),
        );
        state.moved_chain.push((a, b));
        assert_eq!(state.chase_moved(a), b, "one hop");
        state.moved_chain.push((b, a2));
        assert_eq!(state.chase_moved(a), a2, "two hops");
        assert_eq!(state.chase_moved(b), a2, "mid-chain entry");
    }

    /// A fresh resolution clears the record ([CR#400.7j] is per-effect): ids
    /// recorded by resolution 1 must not chase in resolution 2 (Ephemerate
    /// safety at the mechanism level).
    #[test]
    fn moved_chain_resets_when_a_resolution_begins() {
        let (mut state, a, b) = two_permanents_on_field();
        state.moved_chain.push((a, b));

        // Drive a REAL resolution: mint a vanilla creature spell object and
        // push its `StackEntry` by hand, then resolve it.
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Bear".into(),
            types: vec![Type::Creature.def()],
            power: Some(sylex_core::StatValue::Number(2)),
            toughness: Some(sylex_core::StatValue::Number(2)),
            ..Characteristics::default()
        }));
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let spell = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            paid_costs: vec![],
            copy: false,
        });

        state.resolve_object(spell);

        assert!(
            state.moved_chain.is_empty(),
            "a fresh resolution clears the move record ([CR#400.7j] is per-effect)"
        );
    }

    /// Targets are structural ability data rather than an effect wrapper
    /// ([CR#115.1,601.2c]).

    /// Mints a stack-zone spell object (player 0) whose printed face carries
    /// exactly `types`, for `is_permanent_spell` fixtures.
    fn spell_with_types(state: &mut GameState, types: Vec<sylex_core::TypeDef>) -> ObjectId {
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Spell".into(),
            types,
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        state
            .objects
            .mint(ObjectSource::Card(card_id), PlayerId(0), Some(Zone::Stack))
    }

    /// [CR#608.3]: a permanent spell enters the battlefield on resolution.
    /// Baseline: a creature spell (Grizzly-Bears-shaped) is permanent; an
    /// instant spell (bolt-shaped) is not. `Type::def()` carries the correct
    /// `permanent_type` flag, so no plugin load is needed.
    #[test]
    fn is_permanent_spell_reads_the_permanent_type_flag() {
        let mut state = game();
        let bear = spell_with_types(&mut state, vec![Type::Creature.def()]);
        assert!(
            state.is_permanent_spell(bear),
            "a creature spell is permanent"
        );
        let bolt = spell_with_types(&mut state, vec![Type::Instant.def()]);
        assert!(
            !state.is_permanent_spell(bolt),
            "an instant spell is not permanent"
        );
    }

    /// The real proof `is_permanent_spell` reads `TypeDef.permanent_type` and
    /// not an enum name-list: a NON-canonical type name ("Contraption" — not
    /// one of `Type`'s ten variants, so no hardcoded name-list could ever
    /// recognize it) flagged `permanent_type: true` is treated as permanent.
    /// And the converse pins it isn't secretly still keying off the name: a
    /// `TypeDef` named "Land" — a name a hardcoded list WOULD recognize —
    /// but flagged `permanent_type: false` is NOT treated as permanent.
    #[test]
    fn is_permanent_spell_follows_the_flag_not_a_hardcoded_name_list() {
        let mut state = game();
        let contraption = sylex_core::TypeDef {
            name: "Contraption".into(),
            permanent_type: true,
            confers: vec![].into(),
        };
        let novel = spell_with_types(&mut state, vec![contraption]);
        assert!(
            state.is_permanent_spell(novel),
            "a novel type name flagged permanent_type:true is permanent — no hardcoded \
             name-list could ever recognize \"Contraption\""
        );

        let fake_land = sylex_core::TypeDef {
            name: "Land".into(),
            permanent_type: false,
            confers: vec![].into(),
        };
        let non_permanent_land = spell_with_types(&mut state, vec![fake_land]);
        assert!(
            !state.is_permanent_spell(non_permanent_land),
            "a TypeDef named \"Land\" but flagged permanent_type:false is NOT permanent — \
             the old hardcoded name-list would have said true for this name alone"
        );
    }

    /// Land witness for the cast-resolution helper: Land is a Permanent Type
    /// ([CR#110.4]), so a Land-flagged spell fixture reads `permanent_type:
    /// true` here exactly like Creature. The [CR#110.4b] carve-out — a land
    /// card is never a Permanent Spell — is enforced upstream of this helper:
    /// a land is played, not cast ([CR#305.1]), so a real land never goes on
    /// the stack and never becomes the `id` this function is called on.
    /// `is_permanent_spell` itself has no Land special case, and needs none.
    #[test]
    fn land_reads_permanent_type_true_though_a_real_land_never_reaches_this_helper() {
        let mut state = game();
        let land = spell_with_types(&mut state, vec![Type::Land.def()]);
        assert!(
            state.is_permanent_spell(land),
            "Land is a Permanent Type [CR#110.4]; the flag alone drives this helper"
        );
    }

    // ========================================================================
    // Unbuilt: uncalled coin flips, dice rolls, random discard — the work-item
    // machinery drawing from the seeded rng (`engine-randomness` Task 3).
    // ========================================================================

    /// The loop element's product tracks the element's source ([CR#120.3]): a
    /// card/token element carries its LKI snapshot, so reads survive its
    /// removal; a player proxy is zoneless and carries none. Re-spelled from
    /// `it_binding_kind_distinguishes_player_and_object` — `ItBinding` and
    /// `RefKind` left with the discourse record, and the distinction is now
    /// the shape of the value the loop region's element parameter holds.
    #[test]
    fn loop_element_products_distinguish_player_and_object() {
        let (state, creature) = bear_on_field();
        let player = state.player(PlayerId(0)).object;
        let frame = crate::test_support::frame_src(&state, creature);
        let body = sylex_core::Region::new(
            Arc::from([sylex_core::Param {
                def: sylex_core::DefId(0),
                kind: sylex_core::Kind::Entity,
                provenance: sylex_core::Provenance::LoopElement,
            }]),
            sylex_core::Block::default(),
        );
        let element = |id| {
            let mut sub = frame.clone();
            sub.activation = state.enter_loop_region(&body, &frame, id, None);
            state.eval_reference_product(&sylex_core::Reference::Reg(sylex_core::RefId(0)), &sub)
        };

        let object = element(creature);
        assert_eq!(
            object.current,
            Some(creature),
            "a creature element reads back as that object"
        );
        assert!(
            object.lki.is_some(),
            "an object element carries its LKI snapshot"
        );

        let ply = element(player);
        assert_eq!(
            ply.current,
            Some(player),
            "a player element reads back as that player's proxy"
        );
        assert!(
            ply.lki.is_none(),
            "a player element is zoneless — no snapshot"
        );
    }

    /// Targets are structural ability data rather than an effect wrapper
    /// ([CR#115.1,601.2c]).
    #[test]
    fn spell_targets_are_explicit() {
        let spec = sylex_core::TargetSpec::Target(
            sylex_core::Quantity::one(),
            Arc::new(sylex_core::Region::candidate(
                sylex_core::Predicate::creature(),
            )),
        );
        let ability = sylex_core::SpellAbility {
            ability_word: None,
            cost: sylex_core::Cost([].into()),
            targets: vec![spec.clone()].into(),
            effect: sylex_core::Instruction::act(sylex_core::Action::deal_damage(
                sylex_core::Reference::Reg(sylex_core::RefId(6)),
                sylex_core::Count::Literal(3),
            ))
            .into(),
        };
        assert_eq!(ability.targets.as_ref(), std::slice::from_ref(&spec));
    }
}
