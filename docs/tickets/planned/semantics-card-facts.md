---
needs: [semantics-rust-crate, lean-proofs-spec-catalog]
---
# `semantics-card-facts`: crate rename, `Refusal`, context-free AST queries, and card-frame laws

Standard constraints apply. Context-free AST query and card-frame/shape validation ticket of
`semantics-rust-rewrite`: rename `crates/semantics` to `crates/deckmaste_semantics_v3`, define the
shared `Refusal` taxonomy, and implement the `DRS`-free AST queries (`NumberSlots`, CDA/ability/
keyword queries) and static card-frame/shape checks (`CardFacts`, `Card` frame laws, `Keywords`,
`TextChanges`, and the verified `DRS`-free subset of `PredicateShape` / `LocalShape`) in
`crates/deckmaste_semantics_v3` so both `deckmaste_discourse` and `deckmaste_lowering_v3` consume
them directly.

## Scope & Design Decisions

1. **Crate rename and shared `Refusal` taxonomy (`Refusal.lean`)**:
   - Rename `crates/semantics` (package `semantics`) to `crates/deckmaste_semantics_v3` (package
     `deckmaste_semantics_v3`), updating `Cargo.toml`, `crates/xtask`, and
     `.github/workflows/ci.yml`.
   - Define `Refusal` (`Refusal.lean`) in `crates/deckmaste_semantics_v3` so both
     `deckmaste_semantics_v3` (context-free shape, number-slot, and card-frame checks) and
     `deckmaste_discourse` (context-sensitive DRT endophor checks) emit into one refusal type,
     including a temporary `Refusal::Unsupported` variant (removed in `discourse-corpus-agreement`).

2. **Context-free AST queries (`NumberSlots.lean`, `CardFacts.lean`, `Card.lean`,
   `PredicateShape.lean`)**:
   - `NumberSlots` (`[CR#107.1b]`): Classify every `Amount` slot on `Action`, `StaticSpec`, `Cost`,
     `Replacement`, and `GameEvent` as clamped or signed, and provide CDA detection
     (`defines_pt` / `defined_stats` `[CR#604.3]`), `Ability::category` (`[CR#113.3]`),
     `Action::own_deed`, and `Ability::keyword_instances` (expanding `"protection from each
     [characteristic]"` `[CR#702.16h]`) as inherent methods/modules on
     `deckmaste_semantics_v3::ast`.

3. **Context-free card-frame and shape validation (`Card.lean`, `CardFacts.lean`, `Keywords.lean`,
   `TextChanges.lean`, `PredicateShape.lean`, `LocalShape.lean`)**:
   - **Strict `DRS`-free boundary**: Audit each arm of `PredicateShape.lean`, `LocalShape.lean`, and
     `Phrase.lean` for `drs.` or `ops.` before placing it here. An arm belongs in
     `deckmaste_semantics_v3` if and only if its Lean implementation takes no `DRS` and steps no
     sub-phrase (`ops.np`, `ops.act`, etc.):
     - E.g. `gameGains` / `gameHas` (`designation.gameWide`) is context-free and checked here,
       whereas `Action::GainDesignation` (`Phrase.lean:3868-3873`) and `Predicate::HasDesignation`
       (`Phrase.lean:2988`) resolve their subject / `drs.relativeKind` in `DRS` and belong to
       `discourse-cards-rules`; likewise `DieSides`, `ProducedMana`, `ManaHeld`,
       `CounterKindSource`, `definiteAntecedents`, and `chosenAntecedents` inspect `DRS` and belong
       to `discourse-cards-rules`.
   - `Card` / `CardFacts` frame laws: Enforce context-free card-frame invariants across
     `SingleFaced`, `Split`, `DoubleFaced`, `Flip`, `Adventurer`, `Leveler`, `Prototype`, `Case`,
     `Class`, `Saga`, `Room` (door detection), and modal frames (face name, type line uniqueness
     and subtypes, P/T/loyalty/defense box vs. CDA `*`, and land mana cost).
   - `Keywords` (`[CR#207.2c]`) and `TextChanges` (`[CR#612]`): Validate enumerated ability words,
     keyword term labels, and text-change word replacement sorts (`lean-v3-text-word-sort`).
   - Empty coordination vs. sequence refusal taxonomy: Emit `Refusal::NonEmpty` for list containers
     that allow a single element (`Action::Sequentially`, `Action::Simultaneously`,
     `Predicate::And`, `StaticSpec::Conjunction`, `Cost::Compound`) and `Refusal::AtLeastTwo` for
     coordinations that require at least two operands (`NounPhrase::And`, `NounPhrase::Or`,
     `Condition::And`, `Condition::Or`, `Predicate::Or`, `Cost::Or`). Note that `Predicate::Or([])`
     refuses with `AtLeastTwo` here whereas Lean refused `Predicate.or []` with `.nonEmpty`
     (`Zone.badEmptyOr` / `Composition.badEmpty*`); record that as an expected fixture difference.

4. **Spec catalog scope audit**:
   - Update `docs/semantics-spec-catalog.md`'s fragment scope note: none of the seven flagged
     fragments (*Gabriel Angelfire*, *Contractual Safeguard*, *Boommobile*, *Frontline Heroism*,
     *Don't Blink*, *Celebr-8000*, *Atomwheel Acrobats*) are Alchemy; the first five are standard
     Vintage-legal cards (in scope and ported as tests), while *Celebr-8000* and *Atomwheel
     Acrobats* are *Unfinity* cards and remain tagged as deferred Un-set items per `CLAUDE.md`.

## Tests & Verification

- **Unit & spec-catalog tests**: Port the purely context-free entries of
  `docs/semantics-spec-catalog.md` Part 2 Suites M (Card-frame, box, and cost laws), P1 (Static
  keyword frame/list checks), Q1/Q3/Q4 (Clamped vs. signed `NumberSlots`, printed-box literal
  checks, CDA shape checks, and empty/singleton coordination checks), and static `TextChanges` pins
  as native Rust tests in `crates/deckmaste_semantics_v3`.
