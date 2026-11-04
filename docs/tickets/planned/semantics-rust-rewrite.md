---
needs: [semantics-card-facts, discourse-core, discourse-outcomes, discourse-loops, discourse-conditions, discourse-events-deontics, discourse-cards-rules, discourse-cast-permission-publishes-spell]
---
# `deckmaste_discourse`: DRT reference resolution in Rust

Standard constraints apply. Epic coordinating Stage 2 (`crates/deckmaste_semantics_v3` static facts
and `crates/deckmaste_discourse` DRT endophor resolution):
- **`semantics-card-facts`** (`needs: [semantics-rust-crate, lean-proofs-spec-catalog]`): Rename
  `v1/crates/semantics` to `crates/deckmaste_semantics_v3`, define the shared `Refusal` type, and
  implement context-free AST queries (`NumberSlots` `[CR#107.1b]`, CDA `defines_pt`/`defined_stats`
  `[CR#604.3]`, `Ability::category`, `Action::own_deed`, `Ability::keyword_instances`) and
  `DRS`-free card-frame/shape validation (`CardFacts` / `Card` frame laws, `Keywords`,
  `TextChanges`, and the verified `DRS`-free subset of `PredicateShape` / `LocalShape`).
- **`discourse-core`** (`needs: [semantics-card-facts, ...]`): Create `crates/deckmaste_discourse`
  and implement the DRT reference state (`DRS`, `Referent`, `ReferentId`, `GramRole`,
  `OwnScopeFrame`), read-order stepping, caller parameter binding (`ExpandedCall` + `ParamBinding`),
  core word resolution (`J1`, `J2`, `D'`, `AC>R1`, `PrincipleA`, `ContrastAnchor`), and top-level
  `Card` / `CardFace` / `Ability` / `Cost` entry points and `cargo xtask discourse check`.
- **`discourse-outcomes`** (`needs: [discourse-core]`): action outcomes (`OutcomeSort`,
  `thatMuch`/`thatMany`), zone-change successors `[CR#400.7]`, stamps, LKI `[CR#608.2h]`, library
  shuffle forgetting, and created token/copy referents.
- **`discourse-loops`** (`needs: [discourse-outcomes]`): `doForEach` plural abstraction
  ($\Sigma X$), loop result publication, `countOf` / `aggregate` reductions, partitives,
  `theRest` / `theOther`, slices, and piles.
- **`discourse-conditions`** (`needs: [discourse-outcomes]`): `doIf` and `doOnlyIf`
  (`StepPhase.preEvent` vs. `.postEvent`), `DidClause` and `"this way"` lookback grounding,
  `which` relative clauses, Participial Descriptions, choice context, and linked-ability
  `textContext` `[CR#607.2a]`.
- **`discourse-events-deontics`** (`needs: [discourse-outcomes]`): `GameEvent` (`does`, `would`,
  `did`, disjunctive and coordinated trigger subjects), `Replacement` (`StepPhase.preEvent`),
  deontic static specs (`cant`, `can`, `must`, `gatedBy`), `closeModalBox`, and duration retention.
- **`discourse-cast-permission-publishes-spell`** (`needs: [discourse-events-deontics]`): publishing
  a `can <who> (cast|play X)` permission's referent as `"that spell"` / `"those spells"`
  `[CR#601.2a]` to later conjuncts.
- **`discourse-cards-rules`** (`needs: [discourse-outcomes]`): `Card::face_context` cost-letter
  seeding, full `Cost::check` (colon boundary `[CR#602.1a]` and cost-handoff performer tracking),
  `DRS`-resolved `DesignationScope` / `DesignationHolder` checks `[CR#903.3]`,
  `definiteAntecedents` / `chosenAntecedents`, `DRS` outcome/choice scope checks, and `RulesTables`
  `DRS` stepping in `crates/deckmaste_discourse`.

The pipeline it belongs to:

- **`deckmaste_semantics_v3`** (`semantics-rust-crate` and `semantics-card-facts`) resolves names by
  expected sort (§5.2), expands Templates and Definitions over the `.syl` syntax tree
  `sylex_syl` parses, and provides the shared `Refusal` type, context-free AST queries
  (`number_slots`, `defines_pt`, `Ability::category`, `keyword_instances`), and `DRS`-free
  card-frame/shape checks. It does no DRT endophor resolution.
- **`deckmaste_discourse`** (this epic) implements the context-sensitive DRT endophor resolution of
  `docs/decisions/semantics.md` §2 and §4 in Rust. Lean is never in the hot path of card
  compilation, so this is a reimplementation, not a reader of Lean output. The deleted
  `deckmaste_lowering_v3` had part of it; recover what is useful from jj history.
- **`deckmaste_lowering_v3`** (`lowering-v3`) translates the `deckmaste_semantics_v3` AST and
  `deckmaste_discourse` `DRS` bindings to the `sylex_core` representation.

Split out of this epic (2026-11-02): `lean-discourse-export` (the oracle),
`discourse-corpus-agreement` (the gate) and `lean-retirement` (deleting Lean). Absorbed into the
split tickets above (2026-11-03): the multi-use parameter resolution fix and the open proof-catalog
items.

## Sequencing (owner, 2026-11-02)

This replaces the 2026-10-28 ruling, which put every remaining `lean-v3-*` ticket first so that
the Lean workbench's semantics would settle before Rust reimplemented them. The workbench's
architecture (the action-clause spine, the Definition/Template split, DRT reference resolution)
is settled; what remained was mostly AST breadth, which done in Lean first is written twice.

- Stages (owner, 2026-11-02, so nothing is ported only to be ripped out and `.syl` is never
  unchecked): `dsl-named-arguments-anywhere` and `dsl-plain-word-names` first, with Lean still
  checking; then `semantics-rust-crate` (stage 1, `deckmaste_semantics_v3`); then this epic
  (stage 2): `deckmaste_discourse`, stepping in read order (below), checked live against Lean; then
  `discourse-corpus-agreement` and `lean-retirement`.
- `lean-v3-monstrosity-adapt-name-the-source` went first because it removes what would otherwise
  be ported. (`lean-v3-sacrifice-performer-controls` was meant to go first too; it needs a new
  restrictor, so it follows.)
- The AST is frozen while they are built: a representation ticket that lands mid-port is done in
  both languages.
- The representation tickets need `lean-retirement` and are done afterwards, in `.syl` and Rust
  only; the open reference-resolution tickets (`discourse-*`) are part of this epic's work.
- `deckmaste_lowering_v3` follows as `lowering-v3`.

## Read order and parameter binding

Settled (owner, 2026-11-02): argument order carries no meaning at a call site. A Template's or
Definition's spelling is one fixed string, so its arguments have one fixed order, the order its
spelling's holes print in; a call may write them in any order (`dsl-named-arguments-anywhere`, for
readability only). `deckmaste_discourse` steps a call's arguments in that fixed order. "<action> if
<condition>" (`doOnlyIf`) and "if <condition>, <action>" (`doIf`) are two spellings, so two
declarations, and the order is read from the spelling: `doIf`'s `postposed` flag and Lean's
order-recovering rules (`ReadOrder.fieldOrders`, the arms that step an intervening condition before
its action) are not ported. Declarations whose printed order differs from their parameter order
need their spelling (or an equivalent fixed order) before `deckmaste_discourse` relies on it.
The read order is the spelling's by default, with per-declaration overrides where resolution order
differs from printed order (owner, 2026-11-02).

- **Order versus timing (`doOnlyIf` vs. `doIf` vs. `withContinuation`).** For a primitive
  constructor inside an `ExpandedCall`, fields that are directly a `Param(ParamBinding)` of the
  enclosing call keep the field positions they occupy in the primitive's own `read_order` and are
  permuted among those positions according to their relative order in the enclosing call's
  `read_order`, while all other fields step in their position in the primitive's `read_order`;
  `Action::DoIf.postposed` is never read (`discourse-core`). When an action's outcomes become
  visible belongs to the body (`StepPhase.preEvent` vs. `.postEvent` in `discourse-conditions`): in
  "<action> if <condition>" (`doOnlyIf`), the condition is checked before the action happens
  [CR#608.2c], so it reads the action as announced (its targets, and its clause's re-mentions of
  earlier referents), not what the action creates or moves
  (`ActionFamilies.postposedCounterConditionReadsTheUnmovedSpell`), while in
  `may(action, ifDid = …)` (`withContinuation`) the follow-up reads what the action moved.
- **Parameter binding (`ExpandedCall` + `ParamBinding`).** `Definition` pushes an `OwnScopeFrame`
  onto `DRS` while `Template` stays transparent. Each declaration's own body is checked once for
  slot-constraint compatibility across all uses of each `NounPhrase` parameter (following the
  parameter through nested calls). At a call site, the first encounter of a `NounPhrase::Param` for
  a `BindingId` resolves the argument in the caller's scope (`OwnScopeFrame`) against those
  combined constraints and binds `id -> Vec<ReferentId>`; every encounter of that `BindingId`
  (first and subsequent) checks the bound referents against its own slot (`discourse-core`).

Still open for after `lean-retirement` (2026-11-02 discussion, not rulings): whether Template
bodies become opaque like Definitions, every argument resolved once at the call site and each use
checked against its own slot; and whether parameters with a default are always passed by name and
required ones never (a corpus audit found required arguments passed by name and optional ones
positionally, many of them verbs like `dealDamage`, `attack` and `block` overloaded across
valencies by defaults).

## Verification

`sylex_syl` and the Lake generator parse the same `.syl` corpus to identical trees
(`dsl-parse-differential`, done). The Lean checker is the oracle for `deckmaste_discourse`, live
while it is built and in the tree for probing until `discourse-corpus-agreement` passes: the
bindings and fixture snapshot come from `lean-discourse-export`, keyed as that ticket says.
`lean-retirement` then deletes Lean.
