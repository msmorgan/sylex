//! The replacement-effect registry ([CR#614]). Gathers replacements watching an
//! event intent and applies them per [CR#616.1] with lineage ([CR#614.5]).
//!
//! Task 1: the `replacement_watches` matcher — does a replacement's `would`
//! (a core `EventFilter`) watch a given live `GameEvent` intent, and who/what
//! does the intent affect.
//!
//! Later tasks (2–10) add: `CantHappen` variant + cant pass, shields registry,
//! `replace_event` loop, `ChooseReplacement` decision, regeneration,
//! umbra/totem armor, genericity proof, and Skip step elision.

use std::sync::Arc;

use sylex_core::Ability;
use sylex_core::Duration;
use sylex_core::EventFilter;
use sylex_core::Prevention;
use sylex_core::Replacement;
use sylex_core::StaticSpec;
use sylex_core::Zone;

use crate::event::AbilityActivated;
use crate::event::Act;
use crate::event::CounterPlaced;
use crate::event::CounterRemoved;
use crate::event::DamageDealt;
use crate::event::DesignationChanged;
use crate::event::GameEvent;
use crate::event::GotDesignation;
use crate::event::LifeGained;
use crate::event::LifeLost;
use crate::event::TokenCreated;
use crate::event::ZoneChange;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::state::GameState;
use crate::trigger::EventPatient;

/// What an intent affects — the object being moved/changed, or the player
/// experiencing the event (e.g. the player drawing a card, gaining life).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Affected {
    Object(ObjectId),
    Player(PlayerId),
}

/// Whether the replacement pipeline intercepts `e` at all ([CR#614]: only
/// INTENTS are replaceable — an event whose apply is still ahead of it.
/// Post-hoc facts (the past-form `ZoneChange`, loss records) and directly
/// recorded facts (`AbilityUsed`) are not). Extending this set is what lifts a
/// master form's would-lane bridge cap: the kinds outside it keep their
/// `would: false` rows — step onsets belong to the Skip elision pass,
/// attach facts follow the already-performed relation mutation, targeting
/// is announce-time legality, coin/die facts carry an already-decided
/// outcome, and tap/untap/attack/block prevention rides the `Cant` statics.
pub(crate) fn replaceable(e: &GameEvent) -> bool {
    matches!(
        e,
        // [CR#701,614.17]: the FUTURE keyword-action event (`committed: false`)
        // is the ONE guardable/replaceable window — a `Cant(Act(…))` suppresses
        // it (indestructible) and a replacement rewrites/replaces it
        // (regeneration, madness, Leyline). The committed PAST fact
        // (`committed: true`), recorded by `FinalizeAct`, opens NO window (this
        // gate refuses it) so a finalized fact never re-triggers cant/replace —
        // the exact analogue of `ZoneChange`'s `snapshot: None` future gate.
        GameEvent::Act(Act {
            committed: false,
            ..
        }) | GameEvent::ZoneChange(ZoneChange { snapshot: None, .. })
            | GameEvent::DamageDealt(DamageDealt { .. })
            | GameEvent::LifeGained(LifeGained { .. })
            | GameEvent::LifeLost(LifeLost { .. })
            | GameEvent::CounterPlaced(CounterPlaced { .. })
            | GameEvent::CounterRemoved(CounterRemoved { .. })
            | GameEvent::SpellCast(_)
            | GameEvent::AbilityActivated(AbilityActivated { .. })
            | GameEvent::TokenCreated(TokenCreated { .. })
            | GameEvent::GotDesignation(GotDesignation { .. })
            | GameEvent::DesignationChanged(DesignationChanged { .. })
    )
}

/// What a replaceable intent AFFECTS — the recipient the [CR#616.1] choice
/// keys on and the `That`/`EventObject` a replacement body reads
/// ([CR#608.2]). `None` for intents with no single recipient (a token spec
/// not yet minted, a game-scope designation flip).
pub(crate) fn affected(e: &GameEvent) -> Option<Affected> {
    match e {
        // `Act(Destroy(x))` (and any object-subject keyword action) affects
        // its first `on` subject — regeneration's replacement body reads it
        // as `That`; a subjectless player-report action (draw/scry/mill)
        // affects its performer ([CR#121.1,616.1]).
        GameEvent::Act(Act { on, who, .. }) => match on.first() {
            Some(&object) => Some(Affected::Object(object)),
            None => who.map(Affected::Player),
        },
        GameEvent::ZoneChange(ZoneChange { object, .. })
        | GameEvent::CounterPlaced(CounterPlaced { object, .. })
        | GameEvent::CounterRemoved(CounterRemoved { object, .. })
        | GameEvent::DamageDealt(DamageDealt { target: object, .. }) => {
            Some(Affected::Object(*object))
        }
        GameEvent::LifeGained(LifeGained { player, .. })
        | GameEvent::LifeLost(LifeLost { player, .. })
        | GameEvent::GotDesignation(GotDesignation { player, .. }) => {
            Some(Affected::Player(*player))
        }
        GameEvent::SpellCast(o) => Some(Affected::Object(*o)),
        GameEvent::AbilityActivated(AbilityActivated { source, .. }) => {
            Some(Affected::Object(*source))
        }
        _ => None,
    }
}

/// Whether `would` (with `Ref(This) = this`, the watching object) watches
/// intent `e` — the Replacement-lane entry into THE one evaluator
/// ([CR#614]): the intercepted intent becomes its fact record
/// ([`crate::eval::FactView`]) and evaluates with participants LIVE (a
/// would-fact's participants are still around by definition), the raw cause
/// triple intact (the `Cause:agent` lift — no lossy pattern re-lift), and
/// `Ref(This)`/`Ref(You)` anchored on the watcher.
pub(crate) fn replacement_watches(
    state: &GameState,
    would: &EventFilter,
    this: ObjectId,
    e: &GameEvent,
) -> bool {
    replacement_watches_with_bindings(
        state,
        would,
        e,
        crate::eval::Bindings::watcher(object_source_of(state, this)),
    )
}

fn replacement_watches_with_frame(
    state: &GameState,
    would: &EventFilter,
    this: ObjectId,
    e: &GameEvent,
    frame: &crate::stack::ExecutionFrame,
) -> bool {
    replacement_watches_with_bindings(
        state,
        would,
        e,
        crate::eval::Bindings {
            watcher: object_source_of(state, this),
            frame: Some(frame),
            shape_only: false,
        },
    )
}

fn replacement_watches_with_bindings(
    state: &GameState,
    would: &EventFilter,
    e: &GameEvent,
    bindings: crate::eval::Bindings<'_>,
) -> bool {
    if !replaceable(e) {
        return false;
    }
    let Some(fact) = crate::eval::FactView::of(state, e) else {
        return false;
    };
    state.eval(would, &fact, crate::eval::Lane::Replacement, &bindings)
}

/// Look through a remembered `EventFilter` macro invocation (`Expanded`) to
/// the underlying structural form.
pub(crate) fn look_through_event(event: &EventFilter) -> &EventFilter {
    event
}

/// [CR#614.17]: whether any battlefield static makes `e` unable to happen.
/// Runs before the replacement registry — can't-happen events are suppressed
/// entirely; the replacement loop is skipped ([CR#614.17c]).
pub(crate) fn cant_event(state: &GameState, e: &GameEvent) -> bool {
    if !replaceable(e) {
        return false;
    }
    let view = state.layers();
    let battlefield_cant = state.zones.battlefield.iter().any(|&obj| {
        crate::legal::object_has_static(&view, obj, &|s| {
            matches!(s, StaticSpec::CantHappen(would)
                if replacement_watches(state, look_through_event(would), obj, e))
        })
    });
    if battlefield_cant {
        return true;
    }
    // [CR#614.17]: a resolved one-shot may ALSO grant a `CantHappen` ROW (a
    // self-filtered instance row — its filter carries its own subject). Anchor
    // `Ref(This)` on the instance controller's stable player proxy (mirrors
    // `gather`'s floating-effect watcher choice); a self-filtered row does not
    // depend on a live source object.
    state.continuous.iter().any(|ce| {
        let anchor = state.player(ce.controller).object;
        ce.rows.iter().any(|s| {
            matches!(s, StaticSpec::CantHappen(would)
                if replacement_watches(state, look_through_event(would), anchor, e))
        })
    })
}

/// The `ObjectSource` of a live object — used to anchor `Ref(This)`.
fn object_source_of(state: &GameState, id: ObjectId) -> ObjectSource {
    state.objects.obj(id).source
}

// ── Floating-replacement registry types (Task 3) ─────────────────────────────

/// Stable identity for a floating replacement instance (a regeneration shield
/// or other "the next time … instead" effect). Used as the lineage key so the
/// [CR#614.5] applied-set can track it across event rewrites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstanceId(pub u32);

/// A floating one-shot/duration-bounded replacement effect ([CR#614.3]):
/// regeneration shields and "the next time …" replacements. Stored in
/// `GameState.shields`; swept at end of turn; a `one_shot` instance is removed
/// when it is the chosen replacement.
#[derive(Debug, Clone)]
pub struct ReplacementInstance {
    pub id: InstanceId,
    pub replacement: Replacement,
    /// The permanent the replacement protects / watches.
    pub subject: ObjectId,
    pub duration: Duration,
    /// If true, consumed on first application ([CR#614.3]).
    pub one_shot: bool,
    /// The object whose static/activated ability created this instance (used
    /// to build the body frame).
    pub source: ObjectId,
}

/// Stable identity for a replacement effect — either a static ability effect
/// slot or a floating instance. Used in the [CR#614.5] lineage set so the same
/// replacement can't be applied twice to the modified event. `pub` so
/// `DecisionPointKind::ChooseReplacement` (and its integration tests) can name
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplacementKey {
    /// A static `StaticSpec::Replacement` at a known ability/effect index.
    Static {
        source: ObjectId,
        ability: usize,
        effect: usize,
    },
    /// A floating one-shot instance in `GameState.shields`.
    Floating(InstanceId),
}

#[derive(Debug, Clone)]
pub(crate) enum ApplicableEffect {
    // Arc-shared: `Replacement` (~928 B) / `Prevention` (~656 B) are large ASTs.
    // `gather_applicable` builds a `Vec<Applicable>` per event, so an unboxed
    // payload made every element ~960 B; the pointer keeps it thin. See
    // `engine-event-size-boxing`.
    Replacement(Arc<Replacement>),
    Prevention(Arc<Prevention>),
}

/// The steps of a replacement choice [CR#616.1a..616.1e], in step order (the
/// derived `Ord`): when two or more replacement and/or prevention effects
/// apply to one event, one from the first non-empty tier must be chosen. Each
/// replacement kind carries its tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ReplacementTier {
    /// [CR#616.1a]: a self-replacement effect ([CR#614.15]).
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "no self-replacement kind is gathered yet; it carries this tier when it lands"
        )
    )]
    SelfReplacement,
    /// [CR#616.1b]: modifies under whose control an object enters the
    /// battlefield.
    #[expect(
        dead_code,
        reason = "no control-on-entry replacement kind is gathered yet; it carries this tier when it lands"
    )]
    EnterUnderControl,
    /// [CR#616.1c]: makes an object enter the battlefield as a copy of
    /// another.
    #[expect(
        dead_code,
        reason = "no enter-as-copy replacement kind is gathered yet; it carries this tier when it lands"
    )]
    EnterAsCopy,
    /// [CR#616.1d]: makes a card enter the battlefield with its back face up.
    #[expect(
        dead_code,
        reason = "no back-face-up replacement kind is gathered yet; it carries this tier when it lands"
    )]
    EnterBackFaceUp,
    /// [CR#616.1e]: every other replacement or prevention effect.
    Any,
}

/// One replacement or prevention effect that is applicable to the current event
/// — the key (for lineage), the effect itself, and the source object.
#[derive(Debug, Clone)]
pub(crate) struct Applicable {
    pub key: ReplacementKey,
    pub effect: ApplicableEffect,
    /// Which [CR#616.1a..616.1e] step the effect belongs to.
    pub tier: ReplacementTier,
    pub source: ObjectId,
    /// The static ability region that declares every register read by the
    /// replacement/prevention and its body. Floating instances are the
    /// Stage-3 capture case and currently carry no declaration prefix.
    pub params: Arc<[sylex_core::Param]>,
}

/// Collect every replacement effect watching intent `e` from:
/// - static abilities on every battlefield object, and
/// - floating instances in `state.shields`.
///
/// Does NOT filter by lineage; callers apply the lineage set.
pub(crate) fn gather_applicable(state: &GameState, e: &GameEvent) -> Vec<Applicable> {
    let mut out = Vec::new();

    // Static replacements on every battlefield object (self- and
    // other-watching). [CR#616.1]: reads the DERIVED ability list
    // ([`derive::derived_abilities_of`]) rather than the printed-only spine,
    // so a CONDITIONALLY-conferred replacement (a lord granting "if this would
    // be destroyed, exile it instead") participates in the window too. The
    // `ai` index is a lineage discriminator only (`ReplacementKey::Static`
    // never re-fetches the ability by index), so the conferred tail's
    // non-index-stable positions are harmless here. Hot path: the derived read
    // recomputes conferrals per candidate per event (perf-last).
    for &obj in &state.zones.battlefield {
        let (abilities, _printed_len, _captures) =
            crate::derive::derived_abilities_of(state, Some(obj), state.objects.obj(obj).source);
        for (ai, ability) in abilities.iter().enumerate() {
            let Ability::Static(s) = ability else {
                continue;
            };
            // `Ability::Static` carries a single `StaticSpec` directly, so
            // the effect index is always 0 — kept in `ReplacementKey::Static`
            // for shape stability (multi-effect abilities used to
            // disambiguate by index).
            if let StaticSpec::Replacement(r) = &s.body
                && replacement_would(state, r, obj, e, &s.params)
            {
                out.push(Applicable {
                    key: ReplacementKey::Static {
                        source: obj,
                        ability: ai,
                        effect: 0,
                    },
                    effect: ApplicableEffect::Replacement(r.clone()),
                    // [CR#616.1e]: a static ability's replacement is a
                    // continuous effect, never a [CR#614.15] self-replacement
                    // ([CR#611.3]).
                    tier: ReplacementTier::Any,
                    source: obj,
                    params: s.params.clone(),
                });
            }
        }
    }

    // [CR#702.35a]: replacements an object's own static abilities apply to
    // ITSELF, even when it is OFF the battlefield. (Not [CR#614.15]
    // self-replacement effects, which replace a resolving spell or
    // ability's own effect.) A static replacement whose source IS the object
    // the event affects functions from whatever zone that object is in —
    // madness's static ability ("if a player would discard THIS card")
    // functions while the card is in a player's hand, which the battlefield
    // sweep above never looks at. Restricted to an
    // off-battlefield affected object (a battlefield one was already covered)
    // and, structurally, to replacements the object watches on ITSELF (their
    // `would` matches an event whose patient is this same object), so an
    // other-watching ability idle in hand contributes nothing.
    //
    // The AS-ENTERS replacements ([CR#614.12] — "enters tapped / with
    // counters / attached to X") are the ONE exception: a permanent's own
    // `enters the battlefield` replacement is applied atomically at MINT
    // (`as_enters_status`, `step.rs`) reading the reminted battlefield id, NOT
    // through this loop (which would fire it against the pre-mint id and
    // double-apply). Excluded here by its `to: Battlefield` destination.
    if let Some(Affected::Object(obj)) = affected(e)
        && !matches!(
            e,
            GameEvent::ZoneChange(ZoneChange {
                to: Zone::Battlefield,
                ..
            })
        )
        && state
            .objects
            .get(obj)
            .is_some_and(|o| o.zone != Some(Zone::Battlefield))
    {
        // DERIVED read (see the battlefield sweep above): a Falkenrath-Gorger-
        // shape static confers madness on an owned off-battlefield card, so the
        // replacement that redirects its discard is a CONFERRED ability
        // the printed-only spine would never surface here.
        let (abilities, _printed_len, _captures) =
            crate::derive::derived_abilities_of(state, Some(obj), state.objects.obj(obj).source);
        for (ai, ability) in abilities.iter().enumerate() {
            if let Ability::Static(s) = ability
                && let StaticSpec::Replacement(r) = &s.body
                && replacement_would(state, r, obj, e, &s.params)
            {
                out.push(Applicable {
                    key: ReplacementKey::Static {
                        source: obj,
                        ability: ai,
                        effect: 0,
                    },
                    effect: ApplicableEffect::Replacement(r.clone()),
                    // [CR#616.1e]: a madness-style replacement watches its
                    // own card, yet it is a static ability's continuous
                    // effect, not a [CR#614.15] self-replacement ([CR#611.3]).
                    tier: ReplacementTier::Any,
                    source: obj,
                    params: s.params.clone(),
                });
            }
        }
    }

    // Floating instances (regeneration shields, etc.). A shield's `subject`
    // was resolved to a concrete object when the shield was created (its
    // captured `That`), so it matches by SUBJECT IDENTITY — independent of how
    // the source ability refers to it — which is why "regenerate target
    // creature" (source ≠ subject) works, not just "regenerate this creature".
    for inst in &state.shields {
        // [CR#701.19c]: an instruction-scoped "can't be regenerated" rider on
        // this destruction skips the subject's regeneration shields entirely —
        // they are NOT applied (and, being unapplied, NOT consumed). Scoped to
        // the destroy intent `Act(Destroy(…))` (the only event regeneration
        // replaces).
        if matches!(e, GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Destroy")
            && state.no_regen_subjects.contains(&inst.subject)
        {
            continue;
        }
        if floating_watches(state, &inst.replacement, inst.subject, e) {
            out.push(Applicable {
                key: ReplacementKey::Floating(inst.id),
                effect: ApplicableEffect::Replacement(Arc::new(inst.replacement.clone())),
                // [CR#616.1e]: a shield replaces a later event, not its creating
                // effect's own ([CR#614.15]).
                tier: ReplacementTier::Any,
                source: inst.source,
                // Stage 3 gives carried floating bodies explicit captures.
                // Their Stage-2 bridge still has the fixed event-role prefix,
                // matching the register ABI produced by lowering.
                params: sylex_core::event_region_params(),
            });
        }
    }

    // Static preventions on every battlefield object (watching damage events).
    if let GameEvent::DamageDealt(DamageDealt {
        source: event_source,
        target: event_target,
        ..
    }) = *e
    {
        for &obj in &state.zones.battlefield {
            // DERIVED read (see the replacement sweep above): a conferred
            // damage prevention participates too — derived ⊇
            // printed, `ai` is lineage-only.
            let (abilities, _printed_len, _captures) = crate::derive::derived_abilities_of(
                state,
                Some(obj),
                state.objects.obj(obj).source,
            );
            for (ai, ability) in abilities.iter().enumerate() {
                let Ability::Static(s) = ability else {
                    continue;
                };
                if let StaticSpec::Prevention(p) = &s.body
                    && prevention_watches(state, p, obj, event_source, event_target)
                {
                    out.push(Applicable {
                        key: ReplacementKey::Static {
                            source: obj,
                            ability: ai,
                            effect: 0,
                        },
                        effect: ApplicableEffect::Prevention(p.clone()),
                        // [CR#616.1e]: a static prevention effect.
                        tier: ReplacementTier::Any,
                        source: obj,
                        params: s.params.clone(),
                    });
                }
            }
        }
    }

    // NOTE: `StaticSpec::ReplaceRoll` ([CR#614.3] Krark's Thumb-family
    // roll-more replacement) is a genuine THIRD replacement family — its own
    // `EventQuery`/effect shape, structurally distinct from `Replacement`'s
    // Instead/Also/Skip triad — but it is deliberately NOT gathered here.
    // `PlayerAction::FlipCoins`/`RollDice` now draw from the seeded rng and
    // emit real `CoinFlipped`/`DieRolled` resolution events through the
    // cant→replace→apply pipeline, so there IS now a live event for
    // `ReplaceRoll` to intercept — this is no longer the "nothing live to
    // intercept" gap it once was. Gathering it here, routing the flip/roll
    // batches through the replacement registry, and surfacing the ignore
    // selection as a decision is scoped to the `engine-replace-roll` ticket. A
    // `StaticSpec::ReplaceRoll` on the battlefield is simply never matched by
    // either `if let` above, so it contributes nothing — never a panic. Krark's
    // Thumb round-trips and renders; its replacement is a documented no-op
    // until `engine-replace-roll` lands.
    out
}

/// Whether replacement `r` (with watcher `source`) watches intent `e` — its
/// `would` (Instead/Also) matches per `replacement_watches`. Returns `false`
/// for `Skip` (handled by the step-elision pass, Task 9) and `Expanded`.
fn replacement_would(
    state: &GameState,
    r: &Replacement,
    source: ObjectId,
    e: &GameEvent,
    params: &Arc<[sylex_core::Param]>,
) -> bool {
    let frame = replacement_frame(state, source, e, params.clone());
    match r {
        Replacement::Instead { would, .. } | Replacement::Also { would, .. } => {
            let matches = replacement_watches_with_frame(state, would, source, e, &frame);
            state.remove_activation_family(frame.activation);
            matches
        }
        Replacement::Skip { .. } => {
            state.remove_activation_family(frame.activation);
            false
        } /* handled in begin_step, Task 9
           * Provenance is erased at `lower` (`sylex_lowering_v1`), so no
           * loaded value reaches here wrapped. The arm survives only because
           * the variant does; `core-demacro` deletes both. */
    }
}

fn replacement_frame(
    state: &GameState,
    source: ObjectId,
    event: &GameEvent,
    params: Arc<[sylex_core::Param]>,
) -> crate::stack::ExecutionFrame {
    let controller = state.objects.obj(source).controller;
    let roles = state
        .event_roles(event)
        .bindings_over(crate::trigger::TriggerBindings {
            this: Some(LkiSnapshot::capture(state, source)),
            ..crate::trigger::TriggerBindings::default()
        });
    let mut frame = state.frame(source, controller);
    state.frame_set_source_lki(&mut frame, roles.this);
    state.frame_set_defending_player(&mut frame, roles.defending_player);
    state.frame_set_event_bindings(
        &mut frame,
        roles.that_object,
        roles.that_player,
        roles.that_patient,
    );
    state.frame_set_event_extras(
        &mut frame,
        roles.event_amount,
        roles.produced_mana,
        roles.crossed,
    );
    frame.activation = state.enter_region(&sylex_core::Region::new(params, ()), &frame);
    frame
}

fn prevention_watches(
    state: &GameState,
    p: &Prevention,
    obj: ObjectId,
    source: ObjectId,
    target: ObjectId,
) -> bool {
    let watcher = Some(state.objects.obj(obj).source);
    let (from, to) = match p {
        Prevention::PreventAll { from, to, .. }
        | Prevention::PreventNext { from, to, .. }
        | Prevention::PreventNextInstance { from, to } => (from, to),
    };
    crate::target::matches_with(state, source, from, watcher)
        && crate::target::matches_with(state, target, to, watcher)
}

/// Whether a FLOATING shield watches intent `e`. A shield's `subject` was
/// resolved to a concrete object when the shield was created (its captured
/// `That`), so matching is by SUBJECT IDENTITY — the `would`'s participant
/// filters (typically `Ref(EventObject)`, which a frameless gather can't
/// re-resolve) are NOT re-evaluated — paired with the event SHAPE (kind +
/// zones/cause/refinements), evaluated through the one evaluator in
/// `shape_only` bindings.
fn floating_watches(
    state: &GameState,
    replacement: &Replacement,
    subject: ObjectId,
    e: &GameEvent,
) -> bool {
    if !replaceable(e) {
        return false;
    }
    if affected(e) != Some(Affected::Object(subject)) {
        return false;
    }
    let would = match replacement {
        Replacement::Instead { would, .. } | Replacement::Also { would, .. } => would,
        Replacement::Skip { .. } => return false,
        // Provenance is erased at `lower` (`sylex_lowering_v1`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
    };
    let Some(fact) = crate::eval::FactView::of(state, e) else {
        return false;
    };
    state.eval(
        would,
        &fact,
        crate::eval::Lane::Replacement,
        &crate::eval::Bindings {
            watcher: object_source_of(state, subject),
            frame: None,
            shape_only: true,
        },
    )
}

// ── The replacement loop + lineage + apply Instead/Also ──────────────────────

/// The outcome of running the [CR#616.1] replacement loop for one event.
#[derive(Debug)]
pub(crate) enum ReplaceOutcome {
    /// No applicable replacement rewrote the event — apply `e` as-is.
    Pass(GameEvent),
    /// The event was replaced to nothing (`Instead` with a body that doesn't
    /// re-emit the event) — skip `apply`.
    Nothing,
    /// Two or more replacements apply to `current` ([CR#616.1]): its affected
    /// player must choose one before the loop goes on. Nothing is surfaced
    /// yet — the caller owns the rest of the occurrence and suspends it whole
    /// ([`surface_choice`]).
    Choose {
        current: GameEvent,
        /// [CR#614.5]: the lineage already spent on `current`'s chain.
        applied: std::collections::HashSet<ReplacementKey>,
        applicable: Vec<Applicable>,
    },
}

/// The [CR#614.5] STARTING lineage a fresh `replace_event` call seeds its
/// local `applied` set with: an ordinary event starts from empty (the common
/// case), but a `GameEvent::Act` may carry a non-empty `inherited` set —
/// planted by `schedule_body` (an `Instead`/`Also` body's re-emitted product)
/// or by a passed aggregate `Batch` window's apply (its contained per-entity
/// futures) — which pre-excludes those keys so the very replacement that
/// produced this event can't be re-caught by it, per [CR#614.5]'s "or any
/// modified events that may replace that event."
fn inherited_seed(e: &GameEvent) -> std::collections::HashSet<ReplacementKey> {
    match e {
        GameEvent::Act(Act { inherited, .. }) => inherited.clone(),
        _ => std::collections::HashSet::new(),
    }
}

/// Run the [CR#616.1] replacement loop for intent `e` with the [CR#614.5]
/// lineage set. Returns the modified event to apply, nothing (replaced away),
/// or the choice the loop stopped at.
///
/// The affected object IS threaded into the body frame as the event
/// `EventObject` (via `schedule_body`), so a body reads it with
/// `Ref(EventObject)` while `This` stays the source ability.
///
/// The [CR#614.5] `applied` lineage does not always start empty: see
/// [`inherited_seed`] — a `GameEvent::Act` can arrive PRE-SEEDED with keys a
/// prior application already spent against its upstream event chain, so this
/// loop (and any further body it schedules, via `apply_one`/`schedule_body`
/// threading the CURRENT accumulated set forward) never re-triggers the same
/// replacement a second time, which [CR#614.5] forbids.
pub(crate) fn replace_event(state: &mut GameState, e: GameEvent) -> ReplaceOutcome {
    // Non-replaceable intents pass through immediately ([CR#614]: only
    // replaceable intents can be modified by replacement effects).
    if !replaceable(&e) {
        return ReplaceOutcome::Pass(e);
    }
    // [CR#614.5]: the lineage set — a replacement that has already been applied
    // to the current event chain (or an ancestor of it, [`inherited_seed`])
    // cannot apply again, terminating loops.
    let applied = inherited_seed(&e);
    replace_loop(state, e, applied)
}

/// The [CR#616.1f] fixpoint over `current` with lineage `applied`: apply the
/// one applicable replacement and look again, until none applies, the event
/// is replaced away, or two or more apply and a player must choose.
fn replace_loop(
    state: &mut GameState,
    mut current: GameEvent,
    mut applied: std::collections::HashSet<ReplacementKey>,
) -> ReplaceOutcome {
    loop {
        let mut applicable: Vec<Applicable> = gather_applicable(state, &current)
            .into_iter()
            .filter(|a| !applied.contains(&a.key))
            .collect();
        if applicable.len() >= 2 {
            return ReplaceOutcome::Choose {
                current,
                applied,
                applicable,
            };
        }
        // [CR#616.1]: no more replacements watch the (possibly modified)
        // event — apply it as-is.
        let Some(a) = applicable.pop() else {
            return ReplaceOutcome::Pass(current);
        };
        // Exactly one applies: no choice to make ([CR#616.1] orders two or
        // more), so it applies.
        applied.insert(a.key);
        match apply_one(state, current, &a, &applied) {
            // Keep looping — the modified event may be watched by further
            // replacements ([CR#616.1f]).
            Some(modified) => current = modified,
            // The event was replaced to nothing (Instead with no re-emitted
            // event).
            None => return ReplaceOutcome::Nothing,
        }
    }
}

/// Apply one replacement to `e`. Returns the modified event to continue
/// looping on, or `None` when the event is replaced to nothing (Instead).
/// Schedules body effects via `schedule_body`, threading `applied` — the
/// CURRENT [CR#614.5] lineage (already including `a`'s own key) — forward so
/// the body's frame carries it as the STARTING set for whatever it resolves
/// into ([`inherited_seed`]).
fn apply_one(
    state: &mut GameState,
    e: GameEvent,
    a: &Applicable,
    applied: &std::collections::HashSet<ReplacementKey>,
) -> Option<GameEvent> {
    // [CR#608.2]: the object the intent affects (the would-be-destroyed
    // permanent, the damaged creature, …) is bound to `That` for the body to
    // read — regeneration heals/taps `That`. `This` stays the source.
    let that = match affected(&e) {
        Some(Affected::Object(id)) => Some(id),
        _ => None,
    };
    let event_amount = intent_magnitude(state, &e);
    match &a.effect {
        ApplicableEffect::Replacement(replacement) => {
            match (**replacement).clone() {
                Replacement::Instead { instead, .. } => {
                    // [CR#614.1a,614.6]: the event is replaced — it does NOT happen.
                    // Schedule the `instead` body; consume a one-shot shield if
                    // present.
                    schedule_body(
                        state,
                        instead,
                        a.source,
                        that,
                        applied,
                        a.params.clone(),
                        event_amount,
                    );
                    // [CR#614.3]: only consume a floating instance when it is one-shot
                    // (e.g. a regeneration shield). Duration-only floating
                    // replacements (one_shot: false)
                    // persist until their duration expires and must
                    // NOT be consumed on use.
                    if let ReplacementKey::Floating(iid) = a.key
                        && state.shields.iter().any(|s| s.id == iid && s.one_shot)
                    {
                        consume_shield(state, iid);
                    }
                    None // event replaced away
                }
                Replacement::Also { also, .. } => {
                    // [CR#614.1c]: the event still happens AND `also` happens.
                    // Schedule the body; the (unchanged) event continues.
                    schedule_body(
                        state,
                        also,
                        a.source,
                        that,
                        applied,
                        a.params.clone(),
                        event_amount,
                    );
                    Some(e)
                }
                Replacement::Skip { .. } => {
                    // Skip is handled by the step-elision pass (Task 9), not
                    // here.
                    Some(e)
                } /* Provenance is erased at `lower` (`sylex_lowering_v1`), so
                   * no loaded value reaches here wrapped. The arm survives only
                   * because the variant does; `core-demacro` deletes both. */
            }
        }
        ApplicableEffect::Prevention(prevention) => {
            if let GameEvent::DamageDealt(DamageDealt {
                source: event_source,
                target: event_target,
                amount,
                combat,
            }) = e
            {
                match prevention.as_ref() {
                    Prevention::PreventAll { .. } => {
                        // [CR#615.6]: prevented damage never happens.
                        None
                    }
                    Prevention::PreventNext { n, .. } => {
                        let frame = replacement_frame(state, a.source, &e, a.params.clone());
                        let n_val = state.eval_count(n, &frame);
                        state.remove_activation_family(frame.activation);
                        if amount <= n_val {
                            None
                        } else {
                            Some(GameEvent::DamageDealt(DamageDealt {
                                source: event_source,
                                target: event_target,
                                amount: amount - n_val,
                                combat,
                            }))
                        }
                    }
                    Prevention::PreventNextInstance { .. } => {
                        // Prevents the entire instance of damage in this event
                        None
                    }
                }
            } else {
                Some(e)
            }
        }
    }
}

/// Schedule an `instead`/`also` body effect as a `RunEffect` work item at
/// the agenda front. The frame is anchored on `source`, with the replaced
/// intent's affected recipient bound for the body to read while `This` stays
/// the source ability. A card-backed recipient binds as `EventObject` (a body
/// reads `Ref(EventObject)` — regeneration heals it, wither/infect put -1/-1
/// counters on it, [CR#702.80a,702.90c]); a PLAYER recipient binds as the
/// kind-poly `EventPatient` — infect gives that player poison counters
/// ([CR#702.90b]). The retired `EventActor` compat alias (a player patient
/// ALSO bound as `that_player`, so an old body could read it via
/// `Ref(EventActor)`) is gone — `EventActor` names the responsible AGENT
/// ([CR#119.9]'s distinction), and a replaced event's affected recipient is
/// never that; `Infect.ron` was the one reader and now reads `EventPatient`.
///
/// `applied` is the [CR#614.5] lineage already spent against the event this
/// body replaces (including the very key that just fired) — threaded into
/// the scheduled frame's activation context (`inherited_replacements`) so a
/// keyword-action window `effect` resolves into starts its OWN
/// `replace_event` loop pre-excluding it ([`inherited_seed`]), rather than
/// being caught by the SAME replacement all over again.
/// The magnitude an amount-carrying intent supplies to a replacement body's
/// `EventAmount` parameter ([CR#107.3], "that many"). An `Instead` replaces the
/// intent away — it never reaches the apply funnel that a trigger's magnitude
/// comes from — so a body that reads "that many" (infect's poison/-1/-1
/// counters, [CR#702.90b,702.90c]; Bruvac's doubled mill, [CR#616.1g,121.2a])
/// must have it supplied HERE, off the replaced intent.
fn intent_magnitude(state: &GameState, e: &GameEvent) -> Option<sylex_core::Uint> {
    state.event_roles(e).event_amount
}

fn schedule_body(
    state: &mut GameState,
    effect: sylex_core::Instruction,
    source: ObjectId,
    that: Option<ObjectId>,
    applied: &std::collections::HashSet<ReplacementKey>,
    params: Arc<[sylex_core::Param]>,
    event_amount: Option<sylex_core::Uint>,
) {
    let controller = state.objects.obj(source).controller;
    // [CR#608.2,608.2k]: bind the affected recipient — the event PATIENT
    // ([CR#120.3]) — so the body can read it while `This` falls back to
    // `source` (`bindings.this` is `None`, the agent never moving off the
    // ability). A frameless body (`that == None`) leaves bindings unset. The
    // recipient is the provenance-explicit `EventPatient`; a card/token
    // recipient ALSO mirrors into `that_object` so `EventObject` bodies keep
    // reading it (regeneration/wither/infect-on-a-creature).
    let (event_object, event_patient) =
        that.map_or((None, None), |id| match state.objects.obj(id).source {
            ObjectSource::Player(p) => (None, Some(EventPatient::Player(p))),
            ObjectSource::Card(_) => {
                let snapshot = LkiSnapshot::capture(state, id);
                (Some(snapshot.clone()), Some(EventPatient::Object(snapshot)))
            }
        });
    let mut frame = state.frame(source, controller);
    state.frame_set_action_context(&mut frame, applied.clone(), false);
    state.frame_set_event_bindings(&mut frame, event_object, None, event_patient);
    // [CR#107.3]: the replaced intent's own magnitude reaches the body through
    // the region's `EventAmount` parameter.
    state.frame_set_event_extras(&mut frame, event_amount, Vec::new(), None);
    frame.activation = state.enter_region(&sylex_core::Region::new(params, ()), &frame);
    state.schedule_front(vec![crate::agenda::WorkItem::RunEffect {
        effect: Arc::new(effect),
        frame,
    }]);
}

/// Remove a one-shot floating replacement instance from the shields list
/// ([CR#614.3]: one-shot instances are consumed on first application).
fn consume_shield(state: &mut GameState, iid: InstanceId) {
    state.shields.retain(|s| s.id != iid);
}

// ── The ChooseReplacement decision ([CR#616.1]) ──────────────────────────────

/// The player who experiences the event — they choose which replacement applies
/// first when multiple are applicable ([CR#616.1]).
pub(crate) fn affected_player(state: &GameState, e: &GameEvent) -> PlayerId {
    match affected(e) {
        Some(Affected::Player(p)) => p,
        // [CR#616.1]: the affected OBJECT's controller when it's in a
        // controller-bearing zone (battlefield/stack — [CR#109.4]: "Only
        // objects on the stack or on the battlefield have a controller");
        // elsewhere (hand/library/graveyard/exile) it has no controller at
        // all, so the request falls back to its OWNER instead
        // ([CR#108.4,108.4a]) — a discarded card's owner orders madness vs.
        // Leyline of the Void (Task 10). `GameObject::controller` is not
        // safe to read blindly off the battlefield/stack: the zone-change
        // pipeline (`step.rs::apply_zone_will_change`) keeps it equal to
        // the owner there, but that is a behavioral guarantee of one code
        // path, not a type-level invariant, so the zone is checked
        // explicitly instead of trusting the field.
        Some(Affected::Object(o)) => {
            state
                .objects
                .get(o)
                .map_or(state.turn.active_player, |x| match x.zone {
                    Some(Zone::Battlefield | Zone::Stack) => x.controller,
                    _ => match x.source {
                        ObjectSource::Card(c) => state.cards.get(c).owner,
                        // A player proxy has no owner distinct from itself
                        // ([CR#109]) — `controller` IS the player, set at mint.
                        ObjectSource::Player(_) => x.controller,
                    },
                })
        }
        None => state.turn.active_player,
    }
}

/// [CR#616.1a..616.1e]: the effects a replacement choice offers — those of
/// the first non-empty tier. Two or more effects apply, so one of them is
/// chosen; a lone effect in that tier is still offered, as a choice with one
/// option (the engine exposes every legal decision, `docs/decisions/engine.md`
/// law 2).
fn first_tier(applicable: &[Applicable]) -> Vec<ReplacementKey> {
    let first = applicable
        .iter()
        .map(|a| a.tier)
        .min()
        .expect("a replacement choice has applicable effects");
    applicable
        .iter()
        .filter(|a| a.tier == first)
        .map(|a| a.key)
        .collect()
}

/// Suspend an occurrence's replacement pass on a choice: store `suspended`
/// in `GameState.replace_state` and surface a
/// `DecisionPointKind::ChooseReplacement` for `suspended.current`, offering
/// the first non-empty [CR#616.1a..616.1e] tier of `applicable`.
///
/// [CR#616.1]: when two or more players must make these choices at the same
/// time, they make them in APNAP order. The members of a batch reach this
/// point one at a time in member order, so their choosers are asked in member
/// order instead; `engine-replacement-choice-apnap` owns the fix.
pub(crate) fn surface_choice(
    state: &mut GameState,
    suspended: crate::state::ReplaceState,
    applicable: &[Applicable],
) {
    assert!(
        state.replace_state.is_none(),
        "one replacement pass is suspended at a time — the pending choice blocks every other apply"
    );
    let chooser = affected_player(state, &suspended.current);
    let offered = first_tier(applicable);
    state.replace_state = Some(suspended);
    state.pending = Some(crate::decide::DecisionPointKind::ChooseReplacement(
        crate::decide::pending::ChooseReplacement {
            chooser,
            applicable: offered,
        },
    ));
}

/// Run an occurrence's members through the cant ([CR#614.17]) and replacement
/// ([CR#616.1]) passes in member order, appending each surviving member's
/// final event to `resolved`. No member applies here: the whole occurrence
/// is replaced before any of it happens, then commits as one
/// ([CR#603.2c]). Returns the final events, ready to commit, or `None` when
/// a member needs a replacement choice — the occurrence is then suspended
/// whole ([`crate::state::ReplaceState`]) with the choice pending.
///
/// A member whose current-only recipient departed after it was queued is
/// dropped ([CR#608.2b]).
pub(crate) fn replace_members(
    state: &mut GameState,
    mut resolved: Vec<GameEvent>,
    members: Vec<GameEvent>,
    commit: crate::state::OccurrenceCommit,
) -> Option<Vec<GameEvent>> {
    let mut members = members.into_iter();
    while let Some(e) = members.next() {
        if !state.event_current_recipient_is_live(&e) || cant_event(state, &e) {
            continue;
        }
        match replace_event(state, e) {
            ReplaceOutcome::Pass(final_event) => resolved.push(final_event),
            ReplaceOutcome::Nothing => {}
            ReplaceOutcome::Choose {
                current,
                applied,
                applicable,
            } => {
                let suspended = crate::state::ReplaceState {
                    resolved,
                    current,
                    applied,
                    remaining: members.collect(),
                    commit,
                };
                surface_choice(state, suspended, &applicable);
                return None;
            }
        }
    }
    Some(resolved)
}

/// Resume a suspended replacement pass after its `ChooseReplacement`
/// decision. Called from `submit_decision` with the chosen key and the
/// suspended `ReplaceState`.
///
/// Applies the chosen replacement to `rs.current`, finishes that member's
/// replacement loop ([CR#616.1f]), and walks on through the tail. A choice
/// anywhere in there suspends again, carrying every member forward; once the
/// pass completes, the whole occurrence is scheduled to commit at the agenda
/// front as one occurrence in member order.
pub(crate) fn resume_replacements(
    state: &mut GameState,
    rs: crate::state::ReplaceState,
    chosen_key: ReplacementKey,
) {
    let crate::state::ReplaceState {
        mut resolved,
        current,
        mut applied,
        remaining,
        commit,
    } = rs;
    // Find the chosen applicable from a fresh gather, filtered to the key the
    // player picked.
    let chosen = gather_applicable(state, &current)
        .into_iter()
        .find(|a| a.key == chosen_key)
        .expect("chosen replacement key must still be applicable");
    applied.insert(chosen_key);
    if let Some(modified) = apply_one(state, current, &chosen, &applied) {
        // The modified event may be watched by further replacements
        // ([CR#616.1f]).
        match replace_loop(state, modified, applied) {
            ReplaceOutcome::Pass(final_event) => resolved.push(final_event),
            ReplaceOutcome::Nothing => {}
            ReplaceOutcome::Choose {
                current,
                applied,
                applicable,
            } => {
                let suspended = crate::state::ReplaceState {
                    resolved,
                    current,
                    applied,
                    remaining,
                    commit,
                };
                surface_choice(state, suspended, &applicable);
                return;
            }
        }
    }
    if let Some(events) = replace_members(state, resolved, remaining, commit) {
        state.schedule_front(vec![crate::agenda::WorkItem::CommitReplaced {
            events,
            commit,
        }]);
    }
}

#[cfg(test)]
pub(crate) mod tests_support {
    use std::sync::Arc;

    use sylex_card::Card;
    use sylex_card::CardFace;
    use sylex_card::Characteristics;
    use sylex_core::Ability;
    use sylex_core::StaticSpec;
    use sylex_core::Type;
    use sylex_core::Zone;

    use crate::layer::LayeredView;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    /// A minimal two-player game state with a single vanilla 2/2 creature
    /// (player 0's) on the battlefield. No plugins loaded — the card is
    /// synthesised in Rust.
    ///
    /// Returns `(state, view, id)`. Because `LayeredView` is owned, returning
    /// it alongside the state is safe (it holds computed data, not a
    /// borrow).
    pub(crate) fn lone_creature() -> (GameState, LayeredView, ObjectId) {
        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let id = mint_creature_on_battlefield(&mut state);
        let view = state.layers();
        (state, view, id)
    }

    /// Mint a synthetic vanilla 2/2 creature on the battlefield for player 0
    /// and return its `ObjectId`.
    pub(crate) fn mint_creature_on_battlefield(state: &mut GameState) -> ObjectId {
        let card = Arc::new(Card::Normal(CardFace::from(Characteristics {
            name: "Test Creature".into(),
            types: vec![Type::Creature.def()],
            ..Characteristics::default()
        })));
        let card_id = state.cards.push(card, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// Mint a synthetic card straight into `owner`'s hand, with `controller`
    /// set INDEPENDENTLY of `owner` — the real zone-change pipeline
    /// (`step.rs::apply_zone_will_change`) always mints a hand/library/
    /// graveyard/exile object with `controller == owner` ([CR#110.2,108.4]),
    /// but nothing in the type system enforces that off the one chokepoint,
    /// so a test needs to be able to construct the mismatched case directly
    /// to prove [`affected_player`] doesn't blindly trust `controller` there.
    /// Returns `(state, id)`.
    pub(crate) fn card_in_hand(owner: PlayerId, controller: PlayerId) -> (GameState, ObjectId) {
        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let card = Arc::new(Card::Normal(CardFace::from(Characteristics {
            name: "Test Card".into(),
            ..Characteristics::default()
        })));
        let card_id = state.cards.push(card, owner);
        let id = state
            .objects
            .mint(ObjectSource::Card(card_id), controller, Some(Zone::Hand));
        state.zones.hands[owner.index()].push(id);
        (state, id)
    }

    /// Mint a synthetic creature carrying a single `StaticSpec` on the
    /// battlefield for player 0. Returns `(state, id)`.
    pub(crate) fn creature_with_static(effect: StaticSpec) -> (GameState, ObjectId) {
        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let card = Arc::new(Card::Normal(CardFace::from(Characteristics {
            name: "Test Creature".into(),
            types: vec![Type::Creature.def()],
            abilities: vec![Ability::r#static(effect)],
            ..Characteristics::default()
        })));
        let card_id = state.cards.push(card, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        (state, id)
    }
}

#[cfg(test)]
mod tests {
    use sylex_core::Agency;
    use sylex_core::CausePattern;
    use sylex_core::EventFilter;
    use sylex_core::Predicate;
    use sylex_core::Reference;
    use sylex_core::Zone;

    use super::*;
    use crate::decide::DecisionPointKind;
    use crate::event::Cause;
    use crate::event::GameEvent;

    /// An `Act(Destroy(Ref(This)))` would (regeneration's watch) watches the
    /// `Act(Destroy)` keyword-action intent when `this` is the dying object.
    #[test]
    fn destroyed_would_watches_will_destroy_of_self() {
        let (state, _view, id) = super::tests_support::lone_creature();
        let would = EventFilter::Act {
            verb: sylex_core::VerbName::from("Destroy"),
            who: Predicate::Any,
            on: Predicate::Ref(Reference::Reg(sylex_core::RefId(0))),
            cause: None,
        };
        let e = GameEvent::Act(Act {
            verb: sylex_core::VerbName::from("Destroy"),
            who: None,
            on: vec![id],
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: Some(Cause::destroy(Agency::StateBasedAction, None)),
            committed: false,
            contents: None,
            batch: None,
            inherited: std::collections::HashSet::new(),
            contained: false,
        });
        assert!(replacement_watches(&state, &would, id, &e));
    }

    /// A sacrifice is NOT watched by a destruction `Act(Destroy(…))` would — a
    /// sacrifice is a future-form `ZoneChange`, never the Destroy keyword
    /// action ([CR#701.21a]).
    #[test]
    fn destroyed_would_does_not_watch_sacrifice() {
        let (state, _view, id) = super::tests_support::lone_creature();
        let would = EventFilter::Act {
            verb: sylex_core::VerbName::from("Destroy"),
            who: Predicate::Any,
            on: Predicate::Ref(Reference::Reg(sylex_core::RefId(0))),
            cause: None,
        };
        let e = GameEvent::ZoneChange(ZoneChange {
            snapshot: None,
            object: id,
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: Some(Cause::sacrifice(Agency::EffectInstruction, None)),
        });
        assert!(!replacement_watches(&state, &would, id, &e));
    }

    /// An object carrying `CantHappen(Act(Destroy(Ref(This))))`
    /// (indestructible) makes its own `Act(Destroy)` "can't happen"
    /// ([CR#614.17,702.12b]).
    #[test]
    fn cant_happen_suppresses_own_destruction() {
        let (state, id) = super::tests_support::creature_with_static(
            sylex_core::StaticSpec::CantHappen(EventFilter::Act {
                verb: sylex_core::VerbName::from("Destroy"),
                who: Predicate::Any,
                on: Predicate::Ref(Reference::Reg(sylex_core::RefId(0))),
                cause: None,
            }),
        );
        let e = GameEvent::Act(Act {
            verb: sylex_core::VerbName::from("Destroy"),
            who: None,
            on: vec![id],
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
            committed: false,
            contents: None,
            batch: None,
            inherited: std::collections::HashSet::new(),
            contained: false,
        });
        assert!(cant_event(&state, &e));
    }

    /// Lane-split matching ([CR#616.1,616.1f]) — the stacked-madness auto-guard
    /// and the Megrim-under-madness name-fact, in ONE fixture. A `Discard`
    /// would matches a PRISTINE discard (Hand → Graveyard) in the Replacement
    /// lane; once a first replacement has REDIRECTED the content (→ Exile) the
    /// same would no longer matches in the Replacement lane (the master form's
    /// canonical shape no longer holds — a second same-shaped replacement is
    /// inapplicable, [CR#616.1f], with zero semantic guard); yet the redirected
    /// event STILL matches in the Trigger lane, which reads the finalized
    /// name-fact only ([CR#701.9c], "whenever you discard" fires on a discard
    /// gone to exile).
    #[test]
    fn act_would_lane_content_guards_while_trigger_lane_matches_by_name() {
        let (state, _view, id) = super::tests_support::lone_creature();
        let would = EventFilter::Act {
            verb: sylex_core::VerbName::from("Discard"),
            who: Predicate::Any,
            on: Predicate::Any,
            cause: None,
        };
        let discard_to = |to: Zone| {
            GameEvent::Act(Act {
                verb: sylex_core::VerbName::from("Discard"),
                who: None,
                on: vec![id],
                from: Some(Zone::Hand),
                to: Some(to),
                cause: None,
                committed: false,
                contents: None,
                batch: None,
                inherited: std::collections::HashSet::new(),
                contained: false,
            })
        };
        let pristine = discard_to(Zone::Graveyard);
        let redirected = discard_to(Zone::Exile);

        // Would-lane: pristine matches, redirected auto-guards out.
        assert!(
            replacement_watches(&state, &would, id, &pristine),
            "a pristine Hand → Graveyard discard matches its would"
        );
        assert!(
            !replacement_watches(&state, &would, id, &redirected),
            "a discard whose content went → Exile no longer matches — the \
             stacked-madness auto-guard"
        );

        // Trigger-lane: the redirected event still matches by NAME.
        let watcher = object_source_of(&state, id);
        let bindings = crate::eval::Bindings::watcher(watcher);
        let fact = crate::eval::FactView::of(&state, &redirected).expect("discard fact");
        assert!(
            state.eval(&would, &fact, crate::eval::Lane::Trigger, &bindings),
            "the trigger lane reads the name-fact only — a redirected discard \
             still fires \"whenever you discard\""
        );
    }

    /// The `Cause:agent` lift in would lanes ([CR#603.2]): would-facts keep
    /// the raw engine cause triple, so an agent-narrowed pattern evaluates
    /// against the LIVE causing object — and an agentless cause (a
    /// turn-based/state-based action) still fails it.
    #[test]
    fn would_cause_agent_narrows_by_the_live_agent() {
        let (state, _view, id) = super::tests_support::lone_creature();
        let would = EventFilter::ZoneChange {
            what: Predicate::Any,
            from: None,
            to: None,
            cause: Some(sylex_core::Cause::Cause(sylex_core::CausePattern {
                verb: None,
                agency: None,
                agent: Some(Predicate::creature()),
            })),
        };
        // A cause-bearing ZoneChange-view intent (the `Act(Destroy)` filter
        // narrows by patient, not agent, so the agent lift is exercised on the
        // still-cause-bearing future-form `ZoneChange` — same would-lane
        // mechanism).
        let intent = |agent| {
            GameEvent::ZoneChange(ZoneChange {
                snapshot: None,
                object: id,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard,
                enters: None,
                position: None,
                face: None,
                cause: Some(crate::event::Cause::destroy(
                    sylex_core::Agency::EffectInstruction,
                    agent,
                )),
            })
        };
        assert!(
            replacement_watches(
                &state,
                &would,
                id,
                &intent(Some((id, crate::player::PlayerId(0))))
            ),
            "a creature agent satisfies the agent narrow"
        );
        assert!(
            !replacement_watches(&state, &would, id, &intent(None)),
            "an agentless cause fails an agent-narrowed pattern"
        );
    }

    /// The non-intent-master-form lift in would lanes ([CR#601.2i,614]): a
    /// `CantHappen(Cast(who: Ref(You)))` static suppresses its controller's
    /// own cast intent and leaves an opponent's alone.
    #[test]
    fn cant_happen_cast_suppresses_matching_casts() {
        let (mut state, _watcher) = super::tests_support::creature_with_static(
            sylex_core::StaticSpec::CantHappen(EventFilter::Cast {
                who: Predicate::Ref(Reference::Reg(sylex_core::RefId(1))),
                what: Predicate::Any,
            }),
        );
        // A spell object per caster (the fact record's actor is its
        // controller).
        let mut spell = |controller: crate::player::PlayerId| {
            let card = Arc::new(sylex_card::Card::Normal(sylex_card::CardFace::from(
                sylex_card::Characteristics {
                    name: "Test Spell".into(),
                    types: vec![sylex_core::Type::Sorcery.def()],
                    ..sylex_card::Characteristics::default()
                },
            )));
            let cid = state.cards.push(card, controller);
            state
                .objects
                .mint(ObjectSource::Card(cid), controller, Some(Zone::Stack))
        };
        let own = spell(crate::player::PlayerId(0));
        let opponents = spell(crate::player::PlayerId(1));
        assert!(
            cant_event(&state, &GameEvent::SpellCast(own)),
            "the controller's own cast can't happen"
        );
        assert!(
            !cant_event(&state, &GameEvent::SpellCast(opponents)),
            "an opponent's cast is not watched by Ref(You)"
        );
    }

    /// A creature with a static umbra-style `Instead` on itself, plus a
    /// floating shield on it → both gathered for its `Act(Destroy)`.
    #[test]
    fn gather_collects_static_and_floating_for_will_destroy() {
        use sylex_core::Duration;
        use sylex_core::Instruction;
        use sylex_core::TurnMarker;

        let instead = sylex_core::Replacement::Instead {
            would: destroyed_self(),
            instead: Instruction::Sequentially(vec![].into()),
        };
        let (mut state, id) =
            tests_support::creature_with_static(StaticSpec::Replacement(Arc::new(instead.clone())));
        state.shields.push(ReplacementInstance {
            id: InstanceId(0),
            replacement: instead,
            subject: id,
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
            one_shot: true,
            source: id,
        });
        let e = GameEvent::Act(Act {
            verb: sylex_core::VerbName::from("Destroy"),
            who: None,
            on: vec![id],
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: Some(Cause::destroy(Agency::StateBasedAction, None)),
            committed: false,
            contents: None,
            batch: None,
            inherited: std::collections::HashSet::new(),
            contained: false,
        });
        let app = gather_applicable(&state, &e);
        assert_eq!(app.len(), 2);
    }

    /// [CR#616.1] dual-facet gather: because the destroy composite is ONE event
    /// carrying BOTH facets, regeneration (the TAG facet, `Act(Destroy(This))`,
    /// a floating shield on the subject) and a Rest-in-Peace-style graveyard
    /// replacement (the BODY facet, `ZoneChange(→Graveyard)`, a battlefield
    /// static) gather into the SAME applicable-set — the affected player then
    /// orders them. The old two-event split put these on different events
    /// (regen on the `Act`, Rest in Peace on a downstream future-form
    /// `ZoneChange`) = two replace moments; now there is one.
    #[test]
    fn regen_tag_and_graveyard_body_gather_in_one_step() {
        use sylex_core::Duration;
        use sylex_core::Instruction;
        use sylex_core::TurnMarker;

        // Rest in Peace: a battlefield static replacing any `→Graveyard` with
        // exile. It watches the BODY facet of the destroy `Act`.
        let rip = sylex_core::Replacement::Instead {
            would: EventFilter::ZoneChange {
                what: Predicate::Any,
                from: None,
                to: Some(Zone::Graveyard),
                cause: None,
            },
            instead: Instruction::act(sylex_core::Action::move_to(
                Reference::Reg(sylex_core::RefId(2)),
                Zone::Exile,
            )),
        };
        let (mut state, subject) =
            tests_support::creature_with_static(StaticSpec::Replacement(Arc::new(rip)));

        // A regeneration shield on the SUBJECT: watches the TAG facet.
        let regen = sylex_core::Replacement::Instead {
            would: destroyed_self(),
            instead: Instruction::Sequentially(vec![].into()),
        };
        state.shields.push(ReplacementInstance {
            id: InstanceId(0),
            replacement: regen,
            subject,
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
            one_shot: true,
            source: subject,
        });

        let e = GameEvent::Act(Act {
            verb: sylex_core::VerbName::from("Destroy"),
            who: None,
            on: vec![subject],
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: Some(Cause::destroy(Agency::EffectInstruction, None)),
            committed: false,
            contents: None,
            batch: None,
            inherited: std::collections::HashSet::new(),
            contained: false,
        });
        let app = gather_applicable(&state, &e);
        assert_eq!(
            app.len(),
            2,
            "both facets — regen (tag) and RiP (body) — gather on the one Act"
        );
    }

    /// [CR#603.6] dual-facet matching is BODY-shape-gated: a reorder `Act`
    /// (scry, `from`/`to` = `None`) is NOT a zone change, so a
    /// `ZoneChange(→Graveyard)` replacement `would` does not watch it — while
    /// the move-verb destroy `Act` (shape present) does.
    #[test]
    fn graveyard_would_matches_destroy_body_not_scry() {
        let (state, _view, id) = super::tests_support::lone_creature();
        let would = EventFilter::ZoneChange {
            what: Predicate::Any,
            from: None,
            to: Some(Zone::Graveyard),
            cause: None,
        };
        let destroy = GameEvent::Act(Act {
            verb: sylex_core::VerbName::from("Destroy"),
            who: None,
            on: vec![id],
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: Some(Cause::destroy(Agency::EffectInstruction, None)),
            committed: false,
            contents: None,
            batch: None,
            inherited: std::collections::HashSet::new(),
            contained: false,
        });
        let scry = GameEvent::Act(Act {
            verb: sylex_core::VerbName::from("Scry"),
            who: Some(crate::player::PlayerId(0)),
            on: vec![],
            from: None,
            to: None,
            cause: None,
            committed: false,
            contents: None,
            batch: None,
            inherited: std::collections::HashSet::new(),
            contained: false,
        });
        assert!(
            replacement_watches(&state, &would, id, &destroy),
            "a →Graveyard would bites the destroy composite's BODY facet"
        );
        assert!(
            !replacement_watches(&state, &would, id, &scry),
            "a scry Act (no zone shape) is not a →Graveyard zone change"
        );
    }

    /// [CR#701.19c]: an instruction-scoped "can't be regenerated" rider makes
    /// `gather_applicable` SKIP the subject's regeneration shield for its
    /// destroy — and, because the shield is not applied, it is not consumed.
    #[test]
    fn no_regen_rider_skips_regeneration_shield_and_leaves_it_unconsumed() {
        use sylex_core::Duration;
        use sylex_core::Instruction;
        use sylex_core::TurnMarker;

        let instead = sylex_core::Replacement::Instead {
            would: destroyed_self(),
            instead: Instruction::Sequentially(vec![].into()),
        };
        let (mut state, _view, id) = tests_support::lone_creature();
        state.shields.push(ReplacementInstance {
            id: InstanceId(0),
            replacement: instead,
            subject: id,
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
            one_shot: true,
            source: id,
        });
        let e = GameEvent::Act(Act {
            verb: sylex_core::VerbName::from("Destroy"),
            who: None,
            on: vec![id],
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: Some(Cause::destroy(Agency::StateBasedAction, None)),
            committed: false,
            contents: None,
            batch: None,
            inherited: std::collections::HashSet::new(),
            contained: false,
        });
        // Without the rider: the shield is gathered.
        assert_eq!(gather_applicable(&state, &e).len(), 1);
        // With the subject in the no-regen set: skipped, and still present.
        state.no_regen_subjects.push(id);
        assert!(
            gather_applicable(&state, &e).is_empty(),
            "the regeneration shield is not applied"
        );
        assert_eq!(state.shields.len(), 1, "and is not consumed");
    }

    /// The `by`-matcher ([CR#120.3], "damage dealt BY a source"): a `would`
    /// keyed `source: Ref(This)` watches a `DamageDealt` whose SOURCE is
    /// `this`, but NOT one whose source is a different object. This is the
    /// source-side filter infect/wither rely on
    /// ([CR#702.80a,702.90b,702.90c]).
    #[test]
    fn by_matcher_distinguishes_damage_source() {
        let (mut state, _view, source_a) = super::tests_support::lone_creature();
        // A second creature — the "other source".
        let source_b = super::tests_support::mint_creature_on_battlefield(&mut state);
        let target = super::tests_support::mint_creature_on_battlefield(&mut state);

        // "damage dealt BY this creature" — keyed to the watching object.
        let would = EventFilter::Damage {
            source: Predicate::Ref(Reference::Reg(sylex_core::RefId(0))),
            to: Predicate::Any,
            combat: None,
            amount: None,
        };

        // `this == source_a`: damage from A fires the watch; damage from B does
        // not.
        let from_a = GameEvent::DamageDealt(DamageDealt {
            source: source_a,
            target,
            amount: 2,
            combat: false,
        });
        let from_b = GameEvent::DamageDealt(DamageDealt {
            source: source_b,
            target,
            amount: 2,
            combat: false,
        });
        assert!(
            replacement_watches(&state, &would, source_a, &from_a),
            "a `source: Ref(This)` would watches damage from its own source"
        );
        assert!(
            !replacement_watches(&state, &would, source_a, &from_b),
            "it must NOT watch damage from a different source"
        );

        // A bare `source: Any` watches both (the default, source-agnostic).
        let any = EventFilter::Damage {
            source: Predicate::Any,
            to: Predicate::Any,
            combat: None,
            amount: None,
        };
        assert!(replacement_watches(&state, &any, source_a, &from_b));
    }

    /// Helper: the abstract `EventFilter` for "this permanent would be
    /// destroyed" (BF→GY with verb "Destroy"), as used in replacement `would`
    /// fields.
    fn destroyed_self() -> EventFilter {
        // Regeneration's watch: the `Destroy(this)` keyword-action intent.
        EventFilter::Act {
            verb: sylex_core::VerbName::from("Destroy"),
            who: Predicate::Any,
            on: Predicate::Ref(Reference::Reg(sylex_core::RefId(0))),
            cause: None,
        }
    }

    /// A damage-prevention Static Spec prevents damage dealt to the creature.
    #[test]
    fn prevention_effect_prevents_damage() {
        use sylex_core::Predicate;
        use sylex_core::Prevention;
        use sylex_core::Reference;

        let (mut state, id) = super::tests_support::creature_with_static(StaticSpec::Prevention(
            Arc::new(Prevention::PreventAll {
                from: Predicate::Any,
                to: Predicate::Ref(Reference::Reg(sylex_core::RefId(0))),
                duration: None,
            }),
        ));
        let source = super::tests_support::mint_creature_on_battlefield(&mut state);
        let e = GameEvent::DamageDealt(DamageDealt {
            source,
            target: id,
            amount: 3,
            combat: false,
        });
        let app = gather_applicable(&state, &e);
        assert_eq!(app.len(), 1, "prevention is applicable to the damage event");

        let outcome = replace_event(&mut state, e);
        assert!(
            matches!(outcome, ReplaceOutcome::Nothing),
            "damage is fully prevented"
        );
    }

    /// A damage-prevention Static Spec prevents next N damage.
    #[test]
    fn prevention_effect_prevents_next_n_damage() {
        use sylex_core::Count;
        use sylex_core::Predicate;
        use sylex_core::Prevention;
        use sylex_core::Reference;

        let (mut state, id) = super::tests_support::creature_with_static(StaticSpec::Prevention(
            Arc::new(Prevention::PreventNext {
                n: Count::Literal(2),
                from: Predicate::Any,
                to: Predicate::Ref(Reference::Reg(sylex_core::RefId(0))),
                duration: None,
            }),
        ));
        let source = super::tests_support::mint_creature_on_battlefield(&mut state);
        let e = GameEvent::DamageDealt(DamageDealt {
            source,
            target: id,
            amount: 3,
            combat: false,
        });
        let app = gather_applicable(&state, &e);
        assert_eq!(app.len(), 1, "prevention is applicable");

        let outcome = replace_event(&mut state, e);
        if let ReplaceOutcome::Pass(GameEvent::DamageDealt(DamageDealt { amount, .. })) = outcome {
            assert_eq!(amount, 1, "3 damage is reduced by 2 to 1");
        } else {
            panic!("expected Pass with reduced amount, got {outcome:?}");
        }
    }

    /// The [CR#701.21a] entailment, replacement half: a PLAIN to-graveyard
    /// `would` (no cause narrow) intercepts a SACRIFICE intent — the
    /// sacrifice is structurally the entailed Battlefield→Graveyard move,
    /// no per-verb engine arm — while the Destroy-narrowed `would` does not
    /// (see `destroyed_would_does_not_watch_sacrifice`).
    #[test]
    fn graveyard_would_intercepts_a_sacrifice() {
        let (state, _view, id) = super::tests_support::lone_creature();
        let would = EventFilter::ZoneChange {
            what: Predicate::Any,
            from: None,
            to: Some(Zone::Graveyard),
            cause: None,
        };
        let e = GameEvent::ZoneChange(ZoneChange {
            snapshot: None,
            object: id,
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: Some(Cause::sacrifice(Agency::EffectInstruction, None)),
        });
        assert!(
            replacement_watches(&state, &would, id, &e),
            "a graveyard replacement intercepts a sacrifice [CR#701.21a]"
        );
        // And the SACRIFICE-narrowed would matches it too, by its verb.
        let sacrifice_would = EventFilter::ZoneChange {
            what: Predicate::Any,
            from: None,
            to: Some(Zone::Graveyard),
            cause: Some(sylex_core::Cause::Cause(CausePattern {
                verb: Some(sylex_core::VerbName::from("Sacrifice")),
                agency: None,
                agent: None,
            })),
        };
        assert!(replacement_watches(&state, &sacrifice_would, id, &e));
    }

    /// A replacement pass suspended on `current`, the lone member of an
    /// `Occurrence::Single`.
    fn suspended_single(current: GameEvent) -> crate::state::ReplaceState {
        crate::state::ReplaceState {
            resolved: vec![],
            current,
            applied: std::collections::HashSet::new(),
            remaining: vec![],
            commit: crate::state::OccurrenceCommit::Single,
        }
    }

    /// [CR#616.1a,616.1e]: when a self-replacement effect and ordinary
    /// replacement effects all apply to one event, the choice offers only the
    /// self-replacement — one of the first non-empty tier must be chosen. No
    /// self-replacement kind is gathered yet, so the applicable set is built
    /// by hand: a destroy of a creature with two ordinary `Instead`s on it and
    /// one [CR#614.15] self-replacement.
    #[test]
    fn choice_offers_only_the_self_replacement_tier() {
        let (mut state, _view, id) = super::tests_support::lone_creature();
        let applicable = |effect: usize, tier: ReplacementTier| Applicable {
            key: ReplacementKey::Static {
                source: id,
                ability: 0,
                effect,
            },
            effect: ApplicableEffect::Replacement(Arc::new(Replacement::Instead {
                would: destroyed_self(),
                instead: sylex_core::Instruction::Sequentially(vec![].into()),
            })),
            tier,
            source: id,
            params: Arc::from([]),
        };
        let all = [
            applicable(0, ReplacementTier::Any),
            applicable(1, ReplacementTier::SelfReplacement),
            applicable(2, ReplacementTier::Any),
        ];
        let destroy = GameEvent::Act(Act {
            verb: sylex_core::VerbName::from("Destroy"),
            who: None,
            on: vec![id],
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: Some(Cause::destroy(Agency::EffectInstruction, None)),
            committed: false,
            contents: None,
            batch: None,
            inherited: std::collections::HashSet::new(),
            contained: false,
        });
        let offered = |state: &mut GameState, applicable: &[Applicable]| {
            state.replace_state = None;
            surface_choice(state, suspended_single(destroy.clone()), applicable);
            let Some(DecisionPointKind::ChooseReplacement(
                crate::decide::pending::ChooseReplacement { applicable, .. },
            )) = state.pending.take()
            else {
                panic!("expected a surfaced ChooseReplacement decision");
            };
            applicable
        };
        assert_eq!(
            offered(&mut state, &all),
            [all[1].key],
            "only the self-replacement is offered [CR#616.1a]"
        );
        // [CR#616.1e]: with nothing in an earlier tier, every applicable
        // effect is offered.
        assert_eq!(
            offered(&mut state, &[all[0].clone(), all[2].clone()]),
            [all[0].key, all[2].key],
        );
    }

    /// [CR#616.1]: the replacement chooser for an event affecting a card in
    /// HAND is that card's OWNER, not whatever `controller` the object
    /// happens to carry. `controller` is deliberately set to the ACTIVE
    /// player here (a value that would win if `affected_player` naively read
    /// `.controller`) while `owner` is the other player — proving the fix
    /// reads owner off the battlefield/stack rather than trusting
    /// `controller`. Load-bearing for madness-vs-Leyline ordering (Task 10):
    /// the discarded card's owner must pick the replacement order.
    #[test]
    fn choose_replacement_chooser_falls_back_to_hand_owner() {
        // Owned by player 1; `controller` deliberately mismatched to player
        // 0, the active player — the value the OLD `.controller`-trusting
        // code would have returned.
        let (mut state, id) = super::tests_support::card_in_hand(PlayerId(1), PlayerId(0));
        state.turn.active_player = PlayerId(0);

        let two_applicable: Vec<Applicable> = (0..2)
            .map(|effect| Applicable {
                key: ReplacementKey::Static {
                    source: id,
                    ability: 0,
                    effect,
                },
                effect: ApplicableEffect::Replacement(Arc::new(Replacement::Instead {
                    would: EventFilter::ZoneChange {
                        what: Predicate::Any,
                        from: None,
                        to: None,
                        cause: None,
                    },
                    instead: sylex_core::Instruction::Sequentially(vec![].into()),
                })),
                tier: ReplacementTier::Any,
                source: id,
                params: Arc::from([]),
            })
            .collect();

        let e = GameEvent::ZoneChange(ZoneChange {
            snapshot: None,
            object: id,
            from: Some(Zone::Hand),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: Some(Cause::discard(Agency::EffectInstruction, None)),
        });
        surface_choice(&mut state, suspended_single(e), &two_applicable);

        let Some(DecisionPointKind::ChooseReplacement(crate::decide::pending::ChooseReplacement {
            chooser,
            ..
        })) = state.pending
        else {
            panic!("expected a surfaced ChooseReplacement decision");
        };
        assert_eq!(
            chooser,
            PlayerId(1),
            "the card's OWNER chooses off the battlefield, not the active player \
             the (mismatched) controller field was set to"
        );
    }
}
