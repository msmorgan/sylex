//! Minimal characteristics derivation: what abilities an object has. The
//! seed of the stage-4 layer system — in the skeleton, the face's intrinsic
//! printed abilities only; type/subtype-conferred abilities ([CR#305.6] falls
//! out of the data; the engine never special-cases land subtypes) are folded
//! in later, per pass, by `layer::fold_conferred_abilities`.

use std::sync::Arc;

use sylex_card::Card;
use sylex_card::CardFace;
use sylex_core::Ability;
use sylex_core::Action;
use sylex_core::ColorOrColorless;
use sylex_core::CostComponent;
use sylex_core::Count;
use sylex_core::Instruction;
use sylex_core::ManaSpec;
use sylex_core::Uint;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::state::GameState;

type AbilityCaptures = Vec<(sylex_core::RefId, crate::activation::Value)>;
type DerivedAbilities = (Vec<Ability>, usize, Vec<AbilityCaptures>);

/// The face an object presents. Skeleton: the front face.
#[must_use]
pub fn face(card: &Card) -> &CardFace {
    card.primary_face()
}

/// The face an object currently presents on the battlefield
/// ([CR#712.6c,712.6d]): the back face iff it's a two-faced card showing its
/// back (`Side::Back`), else the front. Off the battlefield use `face` (front
/// only, [CR#712.6a]).
#[must_use]
pub fn face_of(state: &GameState, id: ObjectId) -> &CardFace {
    let obj = state.objects.obj(id);
    let card = &state
        .cards
        .get(obj.card_id().expect("card-backed object"))
        .def;
    match (obj.side, card.as_ref()) {
        (crate::object::Side::Back, Card::DoubleFaced { back, .. }) => back,
        _ => face(card),
    }
}

/// The mana value of the card-backed object `id` ([CR#202.3]). A nonmodal
/// double-faced permanent showing its back computes it from its front face's
/// mana cost ([CR#202.3b,712.6d]); every other object from the face it
/// presents. `X` counts as 0 ([CR#202.3e]); the on-stack announced X is not
/// wired.
///
/// # Panics
///
/// Panics on a player proxy, a stale id, or a back-up object whose card is
/// not double-faced.
#[must_use]
pub(crate) fn mana_value_of(state: &GameState, id: ObjectId) -> Uint {
    let obj = state.objects.obj(id);
    let costed = match obj.side {
        crate::object::Side::Front => face_of(state, id),
        crate::object::Side::Back => match state
            .cards
            .get(obj.card_id().expect("card-backed object"))
            .def
            .as_ref()
        {
            Card::DoubleFaced {
                layout: sylex_card::DoubleFacedLayout::Transforming,
                front,
                ..
            } => front,
            Card::DoubleFaced {
                layout: sylex_card::DoubleFacedLayout::ModalDfc,
                back,
                ..
            } => back,
            other @ (Card::Normal(_)
            | Card::Split { .. }
            | Card::Flip { .. }
            | Card::Adventurer { .. }) => {
                panic!("object {id:?} shows its back, but its card has no back face: {other:?}")
            }
        },
    };
    costed.characteristics.mana_cost.mana_value()
}

/// The printed loyalty ([CR#209.1,306.5a]) on the face the card-backed object
/// `id` presents, `None` when that face has none. This is the loyalty
/// characteristic, never the loyalty counter count.
///
/// # Panics
///
/// Panics on a player proxy or a stale id.
#[must_use]
pub(crate) fn loyalty_of(state: &GameState, id: ObjectId) -> Option<sylex_core::Int> {
    crate::layer::base_stat(face_of(state, id).characteristics.loyalty.as_ref())
}

/// A face's INTRINSIC printed abilities only (`face.abilities`) — NOT
/// type/subtype conferrals. Type/subtype conferral is re-derived from the
/// object's CURRENT characteristics each layer pass by
/// `layer::fold_conferred_abilities`, not cached here — which is what makes
/// conferral track layer-4 type/subtype changes, both ADDs and REMOVALS.
/// (Conferred abilities arrive through the ONE emission path,
/// [`Property::conferred_ability`], and occupy the ordinary ability hierarchy
/// — a basic land type's intrinsic mana ability ([CR#305.6]) is visible and
/// layer-6-removable like printed text; that fold happens in
/// `layer::fold_conferred_abilities`, not here.) `Action::ActivateAbility`
/// indexes the
/// [`usable_abilities`] view of this list. Computed once per card at setup
/// (`Cards::push`) and cached on the `CardInstance`.
#[must_use]
pub(crate) fn printed_of_face(face: &CardFace) -> Vec<Ability> {
    face.characteristics.abilities.clone()
}

/// The PRINTED abilities of the face the object presents
/// ([CR#712.6c,712.6d]), from the per-card cache: the same list
/// `layer::base_values` starts from.
///
/// This is the cycle-safe base `layer::gather` reads. External callers
/// should use [`abilities`] to get the layer-6–derived list instead.
///
/// # Panics
///
/// Panics on a player proxy — callers guard on `card_id()` first.
#[must_use]
pub(crate) fn printed_abilities(state: &GameState, id: ObjectId) -> &[Ability] {
    let obj = state.objects.obj(id);
    let card = obj.card_id().expect("card-backed object");
    &state.cards.get(card).face_cache(obj.side).printed
}

/// Predicate-scoped ability conferrals ([Task 2], `ConferralRule`): for each
/// rule in `state.conferral_rules` whose `scope` matches `id`
/// (`crate::matches` — the same predicate-scope matcher `global_sba_rules`
/// uses, `sba.rs`), the ability it contributes via
/// [`Property::conferred_ability`] — the SAME emission path subtype conferral
/// uses, so a predicate-scoped conferral is an ordinary ability exactly like a
/// subtype's ([CR#305.6]).
///
/// Deliberately NOT folded into `layer::base_values`/`gather` (the
/// `layers()` computation itself): `crate::matches` resolves a
/// `Characteristic` scope (e.g. `Type(Planeswalker)`) by calling
/// `state.layers()` (`has_type` et al.), so folding conferral rules INSIDE
/// `layers()` would recurse without bound. Callers that assemble an object's
/// derived ability list from an ALREADY-COMPUTED `LayeredView` ([`abilities`],
/// [`usable_abilities`]) fold this in afterward instead — one extra, self-
/// contained `layers()` recompute per matching id, not a cycle.
#[must_use]
pub(crate) fn conferred_rule_abilities(state: &GameState, id: ObjectId) -> Vec<Ability> {
    state
        .conferral_rules
        .iter()
        .filter(|rule| crate::matches(state, id, &rule.scope))
        .filter_map(|rule| rule.confer.conferred_ability())
        .collect()
}

/// The object's USABLE derived abilities after layer 6 — the indexable
/// surface `Action::ActivateAbility { ability }` points into, shared by
/// `legal_actions`, `decide`'s `ActivateAbility` arm, `begin_activate`, and
/// the render views ("SAME list, SAME order").
///
/// Predicate-scoped `ConferralRule` conferrals ([`conferred_rule_abilities`])
/// are folded in here, alongside the layer view's own list. Identical in
/// content to [`abilities`]: no ability class is hidden from card-facing
/// queries, so the indexable surface and the card-facing surface are ONE list
/// ([CR#113.12] — a rule of the object is not an ability at all, and so is not
/// in either).
#[must_use]
pub fn usable_abilities(state: &GameState, id: ObjectId) -> Arc<Vec<Ability>> {
    abilities(state, id)
}

/// Grant-time captures for one entry in [`usable_abilities`]. Printed and
/// rule-conferred abilities return an empty list. A granted executable root
/// returns the closure environment stored alongside the layer-derived ability.
pub(crate) fn usable_ability_captures(
    state: &GameState,
    id: ObjectId,
    index: usize,
    ability: &Ability,
) -> Vec<(sylex_core::RefId, crate::activation::Value)> {
    let view = state.layers();
    let Some(runtime) = view.ability_runtime(id, index) else {
        return Vec::new();
    };
    runtime
        .flattened
        .iter()
        .find(|entry| entry.ability == *ability)
        .map_or_else(Vec::new, |entry| entry.captures.clone())
}

/// The object's CARD-FACING derived abilities after layer 6
/// ([CR#305.6,613.1f]): base = intrinsic printed abilities; type/subtype
/// conferrals are folded in at layer 4 (`fold_conferred_abilities`); layer 6
/// applies on top. Nothing is filtered: a type-conferred ability is an
/// ordinary ability other cards see and count, while a type's ability-FREE
/// rules ([`sylex_core::Property::Static`], `StateBased`, `TurnBased`) are
/// not abilities at all ([CR#113.12]) and never reach this list.
///
/// Builds a full `LayeredView` per call — fine for a one-shot read (e.g. at
/// resolution), but NEVER call it in a loop: build `state.layers()` once and
/// index the view instead. The layer pipeline itself uses
/// [`printed_abilities`] internally to break the `layers()` →
/// `derive::abilities` → `layers()` recursion.
///
/// Predicate-scoped `ConferralRule` conferrals ([`conferred_rule_abilities`])
/// are folded in here too, the same as a subtype conferral.
#[must_use]
pub fn abilities(state: &GameState, id: ObjectId) -> Arc<Vec<Ability>> {
    let view = state.layers();
    let derived = &view.get(id).abilities;
    let conferred = conferred_rule_abilities(state, id);
    if conferred.is_empty() {
        // Nothing to append — return the shared Arc unchanged (the common
        // case).
        return Arc::clone(derived);
    }
    Arc::new(derived.iter().chain(conferred.iter()).cloned().collect())
}

/// The PRINTED abilities of whatever an `ObjectSource` names — the abilities
/// the trigger scan considers for a watcher. For a card-backed source this is
/// the face's printed list (the same spine that survives reminting and LKI);
/// a player proxy has none. Granted/conferred abilities are folded in by the
/// derived sibling [`derived_abilities_of`] (predicate-scoped conferrals);
/// this printed-only list is the INDEX-STABLE spine that the trigger/
/// replacement scans and their resolution reads key on ([CR#603.2]).
///
/// Composite keywords (ward, prowess) are spliced INLINE: the engine
/// executes the abilities a `KeywordAbility::Composite` carries, so the
/// trigger scan, placement, and resolution all index one flat space.
/// Primitives and other keyword shapes pass through untouched.
#[must_use]
pub fn abilities_of_source(state: &GameState, source: ObjectSource) -> Vec<Ability> {
    match source {
        ObjectSource::Card(card) => {
            // A BATTLEFIELD permanent showing its back sources its printed
            // abilities from the current (back) face ([CR#712.6d]); off the
            // battlefield — any other zone, an LKI/reminted source with no live
            // battlefield object — a DFC has only its front characteristics
            // ([CR#712.6a]). Resolve the source to its live battlefield object
            // (the canonical `ob.source == source` lookup) to read the CURRENT
            // face; else front. Reading the same face on BOTH the placement
            // scan (via [`derived_abilities_of`]) and the
            // resolution-time re-read keeps the fired trigger/
            // replacement `(source, index)` self-consistent.
            let live = state
                .objects
                .iter()
                .find(|ob| ob.source == source && ob.zone == Some(sylex_core::Zone::Battlefield))
                .map(|ob| ob.id);
            let current = match live {
                Some(id) => face_of(state, id),
                None => face(&state.cards.get(card).def),
            };
            let printed = &current.characteristics.abilities;
            let mut out = Vec::with_capacity(printed.len());
            for ability in printed {
                flatten_composites(ability, &mut out);
            }
            out
        }
        ObjectSource::Player(_) => vec![],
    }
}

/// The engine-internal ability enumeration of `source`, EXTENDED with the
/// abilities conferred onto the live object `id` — the DERIVED counterpart of
/// [`abilities_of_source`] that the replacement gather and trigger scan read so
/// a CONDITIONALLY-conferred replacement/triggered ability participates even
/// off the battlefield ([CR#603.2,616.1]). The result is three concatenated
/// sections:
///
/// 1. the PRINTED prefix — byte-for-byte `abilities_of_source(source)`, so a
///    printed ability keeps its enumeration index (the fired-trigger `(source,
///    index)` re-read at resolution stays valid);
/// 2. the predicate-scoped `ConferralRule` tail ([`conferred_rule_abilities`]);
/// 3. the LAYER-DERIVED tail — abilities the [CR#613] layer pipeline GRANTED
///    onto `id` beyond its printed base (`state.layers().get(id).abilities`
///    minus the printed prefix the view seeds from `instance.printed`).
///
/// Sections 2 and 3 both follow the index-stable prefix; they carry no
/// index-stable identity (a fired conferred trigger carries its body by value)
/// and are composite-spliced the same way. The returned
/// `usize` is the length of section 1 only.
///
/// The layer-derived tail is what makes Falkenrath Gorger's printed static
/// ("Each Vampire creature card you own that isn't on the battlefield has
/// madness") FUNCTION: the layer view scopes it exactly — the static is active
/// only while the Gorger is on the battlefield ([CR#611.3b]; `layer::gather`
/// sources statics from battlefield permanents only), and its `Owner(Ref(You))`
/// gate reaches precisely the Gorger controller's OWN off-battlefield Vampires
/// (the `Floating` scope spans objects in every zone). So the granted `Madness`
/// keyword lands on those cards' derived ability lists, and its spliced
/// self-replacement opens the [CR#616.1] discard window here — no companion
/// global `ConferralRule` (which has no source object and so cannot express the
/// owner/Gorger gate) is needed.
///
/// `id` is the live object carrying `source`; `None` (a gone/leaving object,
/// which has no live conferrals or grants) yields the printed list unchanged.
/// Reaches OFF-battlefield objects: both tails derive every card-backed object
/// in ANY zone (the layer base map is not battlefield-only), so an in-hand
/// card's conferred/granted replacement/trigger is enumerated here where
/// `abilities_of_source` alone would miss it.
///
/// PERF: builds `state.layers()` (and, for a game with `ConferralRule`s, the
/// conferral-scope match builds it again) on every call. The gather/scan are
/// hot paths, so this is a real per-event cost — accepted under the project's
/// perf-last priority; no caching added here.
#[must_use]
pub(crate) fn derived_abilities_of(
    state: &GameState,
    id: Option<ObjectId>,
    source: ObjectSource,
) -> DerivedAbilities {
    let mut out = abilities_of_source(state, source);
    let printed_len = out.len();
    let mut captures = vec![Vec::new(); printed_len];
    if let Some(id) = id {
        // Section 2: predicate-scoped `ConferralRule` grants — a global-rule
        // mechanism SEPARATE from the layer system (never folded into
        // `layers()`), so it does not overlap section 3.
        for conferred in conferred_rule_abilities(state, id) {
            let before = out.len();
            flatten_composites(&conferred, &mut out);
            captures.resize(captures.len() + out.len() - before, Vec::new());
        }
        // Section 3: the layer-derived tail. Skip the printed prefix the layer
        // view seeds from `instance.printed` (it is already section 1 in `out`,
        // via `abilities_of_source`) and fold only the appended layer additions
        // — Gorger-style `GainAbility` grants (and any layer-4 type/subtype
        // conferrals). The skip count is the UNFLATTENED printed length,
        // matching the view's unflattened prefix; a shorter derived
        // list (an ability- stripping layer op) simply yields no tail
        // (never-crash). Calling `layers()` here is not a cycle:
        // `derived_abilities_of` is invoked only by the replacement
        // gather and trigger scan, never from inside the
        // layer pipeline, and the pipeline's own matcher (`matches_derived`)
        // never re-enters `layers()`.
        let view = state.layers();
        if let Some(chars) = view.try_get(id) {
            for (index, granted) in chars
                .abilities
                .iter()
                .enumerate()
                .skip(printed_base_len(state, id))
            {
                let runtime = view
                    .ability_runtime(id, index)
                    .expect("derived abilities and runtime companions stay aligned");
                if runtime.flattened.is_empty() {
                    let before = out.len();
                    flatten_composites(granted, &mut out);
                    captures.resize(captures.len() + out.len() - before, Vec::new());
                } else {
                    for entry in &runtime.flattened {
                        out.push(entry.ability.clone());
                        captures.push(entry.captures.clone());
                    }
                }
            }
        }
    }
    (out, printed_len, captures)
}

/// The count of a live object's UNFLATTENED printed abilities — the length of
/// the prefix `layer::base_values` seeds the derived ability list from
/// (`instance.face_cache(side).printed`) before any layer op appends.
/// [`derived_abilities_of`] skips exactly this many entries of the layer view
/// to isolate the layer-granted tail, so this MUST count the object's CURRENT
/// face: a back-up two-faced permanent seeds from its BACK face ([CR#712.6d]),
/// and a stale front-length skip would mis-slice the tail when the faces differ
/// in printed length. A non-card-backed / absent id contributes 0.
fn printed_base_len(state: &GameState, id: ObjectId) -> usize {
    let Some(obj) = state.objects.get(id) else {
        return 0;
    };
    obj.card_id().map_or(0, |card| {
        state.cards.get(card).face_cache(obj.side).printed.len()
    })
}

/// Splice a composite keyword's members into `out` (recursively — a
/// composite may carry another); any other ability passes through as-is.
///
/// This is the ENGINE-INTERNAL enumeration (trigger gathering, the
/// `Has`/keyword reads): a composite's members are executable children of the
/// named container, so they are spliced here and executed. They never enter
/// the carrier's card-facing list — `derive::abilities` reads the layer view's
/// unflattened list, where the container alone stands ([CR#702.21a]: a
/// permanent with ward has ONE ability, ward, not ward plus its trigger).
pub(crate) fn flatten_composites(ability: &Ability, out: &mut Vec<Ability>) {
    visit_flattened(ability, &mut |member| out.push(member.clone()));
}

/// Visit, by reference, each ability [`flatten_composites`] would splice out
/// of `ability`, in the same order, without cloning any of them.
pub(crate) fn visit_flattened(ability: &Ability, visit: &mut impl FnMut(&Ability)) {
    if let Ability::Keyword(k) = ability
        && let Some(members) = composite_members(k)
    {
        for member in members {
            visit_flattened(member, visit);
        }
        return;
    }
    visit(ability);
}

/// The member list of a composite keyword, looked up through the remembered
/// macro invocation; `None` for primitives and other keyword shapes.
fn composite_members(keyword: &sylex_core::KeywordAbility) -> Option<&Vec<Ability>> {
    match keyword {
        sylex_core::KeywordAbility::Composite { abilities, .. } => Some(abilities),
        _ => None,
    }
}

/// Skeleton-subset mana-ability check (a subset of [CR#605.1a]): an activated
/// ability with no targets, cost exactly `[Tap]`, producing a fixed amount
/// of specific mana. The full rule admits more costs and production shapes,
/// but excludes loyalty abilities and any ability whose cost or effect moves a
/// card to or from a library; only self-replacement effects are considered
/// while classifying it. None of those wider shapes are needed here. Returns
/// what it produces. Keyword wrappers are looked through.
#[must_use]
pub fn tap_mana_ability(ability: &Ability) -> Option<(ColorOrColorless, Uint)> {
    if !matches!(
        ability.mana_profile(),
        Some(sylex_core::ActivatedManaProfile::Always)
    ) {
        return None;
    }
    let a = ability.as_activated()?;
    if **a.cost != [CostComponent::Tap] {
        return None;
    }
    match a.effect.body.as_ref() {
        // The produced-mana effect is a bare `AddMana` in RON; the agent is
        // irrelevant for tap-for-mana derivation.
        [
            Instruction::Act {
                action:
                    Action::AddMana(
                        _,
                        Count::Literal(n),
                        sylex_core::ManaProduction::Bare(ManaSpec::Specific(m)),
                    ),
                ..
            },
        ] => Some((*m, *n)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sylex_card::Card;
    use sylex_card::CardFace;
    use sylex_card::Characteristics;
    use sylex_core::Ability;
    use sylex_core::EventFilter;
    use sylex_core::Instruction;
    use sylex_core::Reference;
    use sylex_core::TriggeredAbility;
    use sylex_core::Zone;

    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// [CR#702.21a]: a composite keyword's members are EXECUTABLE CHILDREN of
    /// the named container. `abilities_of_source` (the engine-internal
    /// enumeration the trigger scan consumes) SPLICES them, so ward's
    /// triggered ability is gathered; the container is what the card-facing
    /// list carries, so the trigger never enters it as a second ability.
    #[test]
    fn abilities_of_source_splices_a_composite_keyword_trigger() {
        let mut state = game();
        let trigger = TriggeredAbility {
            ability_word: None,
            where_x: None,
            targets: [].into(),
            from: None,
            event: EventFilter::ZoneChange {
                what: sylex_core::Predicate::Ref(Reference::Reg(sylex_core::RefId(0))),
                from: None,
                to: Some(Zone::Graveyard),
                cause: None,
            },
            condition: None,
            limits: vec![].into(),
            effect: Instruction::draw(
                Reference::Reg(sylex_core::RefId(1)),
                sylex_core::Count::Literal(1),
            )
            .into(),
        };
        let keyword = Ability::Keyword(sylex_core::KeywordAbility::Composite {
            name: sylex_core::Ident::new("Ward"),
            abilities: vec![Ability::triggered(trigger.clone())],
        });
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Composite Triggerer".into(),
            abilities: vec![keyword.clone()],
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));

        let derived = super::abilities_of_source(&state, ObjectSource::Card(card_id));
        assert_eq!(
            derived,
            vec![Ability::triggered(trigger)],
            "abilities_of_source must splice the composite so the trigger scan \
             sees the Triggered member"
        );

        // The card-facing list carries the CONTAINER, never the member.
        let object = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(object);
        assert_eq!(
            *super::abilities(&state, object),
            vec![keyword],
            "the composite's member is not a second card-facing ability"
        );
    }

    /// [Task 2]: a `ConferralRule { scope: Type(Planeswalker), confer:
    /// Property::Ability(...) }` folds into a MATCHING object's derived
    /// ability list ([CR#305.6] pattern, extended from a static subtype to a
    /// live predicate scope) — and NOT into a non-matching object's.
    #[test]
    fn conferred_ability_by_type() {
        use sylex_core::ConferralRule;
        use sylex_core::KeywordAbility;
        use sylex_core::Predicate;
        use sylex_core::Property;
        use sylex_core::Type;

        let mut state = game();
        state.conferral_rules = vec![ConferralRule {
            scope: Predicate::r#type(Type::Planeswalker),
            confer: Property::Ability(Arc::new(Ability::Keyword(KeywordAbility::Trample))),
        }];

        let walker = Card::Normal(CardFace::from(Characteristics {
            name: "Test Walker".into(),
            types: vec![Type::Planeswalker.def()],
            ..Characteristics::default()
        }));
        let walker_card = state.cards.push(Arc::new(walker), PlayerId(0));
        let walker_id = state.objects.mint(
            ObjectSource::Card(walker_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(walker_id);

        let bear = Card::Normal(CardFace::from(Characteristics {
            name: "Test Bear".into(),
            types: vec![Type::Creature.def()],
            ..Characteristics::default()
        }));
        let bear_card = state.cards.push(Arc::new(bear), PlayerId(0));
        let bear_id = state.objects.mint(
            ObjectSource::Card(bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(bear_id);

        assert!(
            super::usable_abilities(&state, walker_id)
                .iter()
                .any(|a| matches!(a, Ability::Keyword(KeywordAbility::Trample))),
            "the planeswalker gains the type-scoped conferred ability"
        );
        assert!(
            !super::usable_abilities(&state, bear_id)
                .iter()
                .any(|a| matches!(a, Ability::Keyword(KeywordAbility::Trample))),
            "a non-planeswalker does not gain the type-scoped conferred ability"
        );
    }

    /// [Task 5b][CR#712.6d]: a BATTLEFIELD permanent showing its back sources
    /// its PRINTED triggered abilities — the trigger-scan spine
    /// `abilities_of_source` — from the BACK face; a front-up (or
    /// off-battlefield) permanent sources them from the front ([CR#712.6a]).
    /// The two faces carry DIFFERENT printed lengths (0 vs 1), so the
    /// derived skip-count (`printed_base_len`, section 3 of
    /// `derived_abilities_of`) is exercised too: the back trigger must land
    /// in the index-stable section-1 prefix (`printed_len == 1`) and appear
    /// EXACTLY once — a stale front-length skip of 0 would fold the layer
    /// view's back trigger a SECOND time.
    #[test]
    fn back_up_permanent_sources_triggers_from_back_face() {
        use sylex_card::Card;
        use sylex_card::CardFace;
        use sylex_card::Characteristics;
        use sylex_card::DoubleFacedLayout;
        use sylex_core::StatValue;
        use sylex_core::Type;
        use sylex_core::Zone;

        use crate::object::Side;

        let back_trigger = TriggeredAbility {
            ability_word: None,
            where_x: None,
            targets: [].into(),
            from: None,
            event: EventFilter::ZoneChange {
                what: sylex_core::Predicate::Ref(Reference::Reg(sylex_core::RefId(0))),
                from: None,
                to: Some(Zone::Graveyard),
                cause: None,
            },
            condition: None,
            limits: vec![].into(),
            effect: Instruction::draw(
                Reference::Reg(sylex_core::RefId(1)),
                sylex_core::Count::Literal(1),
            )
            .into(),
        };
        // Front: vanilla 1/1, ZERO printed abilities. Back: 3/2 with ONE
        // triggered ability the front lacks — distinct printed lengths (0 vs
        // 1).
        let front = CardFace::from(Characteristics {
            name: "Front Vanilla".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(1)),
            toughness: Some(StatValue::Number(1)),
            ..Characteristics::default()
        });
        let back = CardFace::from(Characteristics {
            name: "Back Triggerer".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(3)),
            toughness: Some(StatValue::Number(2)),
            abilities: vec![Ability::triggered(back_trigger.clone())],
            ..Characteristics::default()
        });
        let card = Card::DoubleFaced {
            layout: DoubleFacedLayout::Transforming,
            front,
            back,
        };
        let mut state = game();
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let source = ObjectSource::Card(card_id);
        let id = state
            .objects
            .mint(source, PlayerId(0), Some(Zone::Battlefield));
        state.zones.battlefield.push(id);

        // Front-up: the trigger scan sees the front face — no triggered
        // ability.
        assert!(
            !super::abilities_of_source(&state, source)
                .iter()
                .any(|a| a.as_triggered().is_some()),
            "front-up permanent sources no triggered ability from its vanilla \
             front face [CR#712.6c]"
        );

        // Flip to the back face (Task 5 wires `Transform` to set this; driven
        // directly here, mirroring the layer-view test harness).
        state.objects.obj_mut(id).side = Side::Back;

        // Back-up: the trigger scan sources the back face's trigger
        // [CR#712.6d].
        let triggers: Vec<_> = super::abilities_of_source(&state, source)
            .into_iter()
            .filter(|a| a.as_triggered().is_some())
            .collect();
        assert_eq!(
            triggers,
            vec![Ability::triggered(back_trigger.clone())],
            "back-up permanent sources its back face's triggered ability \
             [CR#712.6d]"
        );

        // Skip-count regression: `derived_abilities_of` is the exact
        // enumeration the trigger scan reads. Section 1 (the
        // index-stable printed prefix) must be the BACK face's
        // (`printed_len == 1`), and the back trigger must
        // appear EXACTLY once — a front-length skip (0) would duplicate the
        // layer view's back trigger into section 3.
        let (derived, printed_len, captures) =
            super::derived_abilities_of(&state, Some(id), source);
        assert_eq!(
            printed_len, 1,
            "section-1 printed length is the back face's"
        );
        assert_eq!(
            derived
                .iter()
                .filter(|a| a.as_triggered().is_some())
                .count(),
            1,
            "the back trigger is enumerated exactly once — neither dropped nor \
             duplicated by the section-3 skip-count"
        );
        assert_eq!(captures.len(), derived.len());
    }

    /// [CR#712.6c,712.6d,202.3b]: a transformed Delver of Secrets derives the
    /// name Insectile Aberration, and its mana value is still computed from
    /// the front face's {U}.
    #[test]
    fn transformed_permanent_derives_its_back_name_and_front_mana_value() {
        use crate::object::Side;

        let canon = sylex_plugin_v1::plugin::Plugin::load_with_sibling_prelude(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../v1/plugins/canon"),
        )
        .unwrap();
        let delver = Arc::new(canon.card("Delver of Secrets").unwrap().core);
        let mut state = game();
        let card_id = state.cards.push(delver, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);

        let view = state.layers();
        assert_eq!(&*view.get(id).name, "Delver of Secrets");
        assert_eq!(view.get(id).mana_value, 1);

        state.objects.obj_mut(id).side = Side::Back;
        let view = state.layers();
        assert_eq!(&*view.get(id).name, "Insectile Aberration");
        assert_eq!(
            view.get(id).mana_value,
            1,
            "a back-up nonmodal DFC takes its mana value from the front's {{U}}"
        );
    }

    /// [CR#712.6d,306.5a]: the loyalty characteristic derives from the face
    /// the permanent presents. A minimal repro of Jace, Vryn's Prodigy // Jace,
    /// Telepath Unbound: a creature front with no loyalty, a planeswalker back
    /// with printed loyalty 5.
    #[test]
    fn back_up_planeswalker_derives_its_back_loyalty() {
        use sylex_card::DoubleFacedLayout;
        use sylex_core::StatValue;
        use sylex_core::Type;

        use crate::object::Side;

        let front = CardFace::from(Characteristics {
            name: "Jace, Vryn's Prodigy".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(0)),
            toughness: Some(StatValue::Number(2)),
            ..Characteristics::default()
        });
        let back = CardFace::from(Characteristics {
            name: "Jace, Telepath Unbound".into(),
            types: vec![Type::Planeswalker.def()],
            loyalty: Some(StatValue::Number(5)),
            ..Characteristics::default()
        });
        let card = Card::DoubleFaced {
            layout: DoubleFacedLayout::Transforming,
            front,
            back,
        };
        let mut state = game();
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);

        assert_eq!(state.layers().get(id).loyalty, None);
        state.objects.obj_mut(id).side = Side::Back;
        assert_eq!(state.layers().get(id).loyalty, Some(5));
    }
}
