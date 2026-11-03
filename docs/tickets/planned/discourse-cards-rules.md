---
needs: [discourse-outcomes]
---
# `discourse-cards-rules`: costs, face context, `DRS`-dependent shape checks, and rules tables

Standard constraints apply. Costs, face context, `DRS`-dependent shape/designation checks, and
rules-table family of `semantics-rust-rewrite` in `crates/deckmaste_discourse` (building on the
context-free AST queries and frame checks from `semantics-card-facts` in
`crates/deckmaste_semantics_v3`).

## Scope & Design Decisions

1. **Card face `DRS` context, costs, and colon boundary (`Card.lean`, `Abilities.lean`,
   `[CR#118.1,602.1a]`)**:
   - `Card::face_context`: Seed mana-cost `{X}` letters (`Letter(l)`) and `sourceTypes` into the
     initial `DRS` and step the face's abilities (`Ability::check_text`), combining with the
     context-free frame checks from `deckmaste_semantics_v3` when `Card::check` runs.
   - Full `Cost::check`: Thread referents, outcomes, and moved-object successors introduced in an
     activation or additional cost (`Cost::Perform(action)`) across the `[Cost]: [Effect]` colon
     boundary `[CR#602.1a]` into the ability's effect `DRS`.
   - **Cost-handoff performer boundary** (`lean-checker-sweep-costs`, `[CR#118.1]`):
     Settled (owner, 2026-11-03): `deckmaste_discourse` is for endophor resolution only, so game
     rules governing who may pay a cost stay out of scope (`lean-checker-sweep-costs`), while
     `Action::Perform` handoffs track the handed-off performer (`isYou == false` for an opponent
     handoff, innermost performer in nested handoffs, and a granted ability keeping its own
     performer in `Actor.badHandedOffLifeCost`, `Actor.badNestedHandoffInnermost`, and
     `Actor.badGrantedAbilityKeepsItsOwnPerformer`).

2. **`DRS`-dependent designation, antecedent, and outcome/choice scope checks
   (`Phrase.lean`, `PredicateShape.lean`, `LocalShape.lean`)**:
   - **Designation scope and holder checks** (`[CR#903.3,101.3,609.3]`,
     `lean-checker-sweep-game-rules`): In `Action::GainDesignation` (`Phrase.lean:3868-3873`) and
     `Predicate::HasDesignation` (`Phrase.lean:2988`), resolve the subject (or `drs.relativeKind`)
     in `DRS` first and then validate designation scope (`Refusal::DesignationScope`) and holder
     kind (`Refusal::DesignationHolder`), refusing `Static.badGainsCommander` (`commander` is
     `.heldByCard`, so granting it to `thisCreature` or an object-resolved pronoun `"it"` refuses
     both `.designationScope` and `.designationHolder`). Settled (owner, 2026-11-03): whether only a
     rule may confer a designation is a game rule, not endophor resolution, and stays out of scope
     (`lean-checker-sweep-game-rules`).
   - `DRS.definiteAntecedents` and `DRS.chosenAntecedents` (`PredicateShape.lean`): Resolve definite
     (`"the <noun>"`) and chosen (`"the chosen <noun>"`) descriptions against visible referents in
     `DRS`.
   - `DRS` outcome/choice scope checks (`LocalShape.lean` and any remaining `drs.`/`ops.`-using
     shape arms): Check `DieSides` (`"those dice"` against `countOutcomes .diceRolled`),
     `ProducedMana` (`countChoice` / `countOutcomes`), `ManaHeld` (`"this mana"` against
     `countOutcomes .manaAdded`), and `CounterKindSource` (`countChoice` / `countOutcomes`).

3. **Rules tables (`Rules.lean`)**:
   - Implement `SbaRule::check`, `Conferral::check`, `ConferralRule::check`,
     `RegistryDefinition::check`, `PredefinedToken::check`, and `RulesTables::check` over `DRS`, and
     wire `RulesTables::check` into `cargo xtask discourse check`.

## Tests & Verification

- **Corpus & fixture gate**: Every corpus card and fixture with no remaining `Refusal::Unsupported`
  agrees with `bindings.jsonl` and `fixtures.jsonl` (plus the documented fixes).
- **Spec catalog**: Port `docs/semantics-spec-catalog.md` Part 1 §9 (Refusal reachability) and the
  `DRS`-dependent entries of Part 2 Suites K (Registry rules), M (Face context, copies, storm/cast
  counts, and M2 representation pins), N (Mana letter/outcome/choice scope checks), P (Keyword
  choices and bodies), Q2 (Shared-subject `gets`/`gains` stepping), and T (Card-fragment pins with
  a distinct claim) as native Rust tests, plus the pins for `Actor.badHandedOffLifeCost`,
  `Actor.badNestedHandoffInnermost`, `Actor.badGrantedAbilityKeepsItsOwnPerformer`,
  `Static.badGainsCommander`, and a pronoun-subject `"it gains commander"` regression pin.
