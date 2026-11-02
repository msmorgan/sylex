---
needs: [semantics-rust-crate, lean-discourse-export, lean-v3-monstrosity-adapt-name-the-source, lean-proofs-spec-catalog, idris-retirement]
---
# [design] `discourse`: DRT reference resolution in Rust

Standard constraints apply. This is an epic, and needs a design dialogue before its `discourse`
work is split into tickets: a core (the reference state and the stepper, spelling-order stepping,
the plain noun-phrase and pronoun cases, and the open questions below) first, then one ticket per
checker family the core's shape allows (Definition scopes, action outcomes and moved objects,
loops and aggregates, conditions, replacements, costs). Each ports the spec catalog entries it
implements as Rust tests.

The pipeline it belongs to:

- **`semantics`** (`semantics-rust-crate`) resolves names by expected sort (§4.5) and expands
  Templates and Definitions over the `.syl` syntax tree `deckmaste_dsl` parses. It does no
  reference resolution.
- **`discourse`** (this epic) implements the DRT referent resolution of
  `docs/decisions/semantics-v3.md` §1 and §3 in Rust. Lean is never in the hot path of card
  compilation, so this is a reimplementation, not a reader of Lean output. The deleted
  `deckmaste_lowering_v3` had part of it; recover what is useful from jj history.
- **`lowering`** (`lowering-v3`) translates the resolved AST to the `deckmaste_core`
  representation.

Split out of this epic (2026-11-02): `lean-discourse-export` (the oracle),
`discourse-corpus-agreement` (the gate) and `lean-retirement` (deleting Lean).

## Sequencing (owner, 2026-11-02)

This replaces the 2026-10-28 ruling, which put every remaining `lean-v3-*` ticket first so that
the Lean workbench's semantics would settle before Rust reimplemented them. The workbench's
architecture (the action-clause spine, the Definition/Template split, DRT reference resolution)
is settled; what remained was mostly AST breadth, which done in Lean first is written twice.

- Stages (owner, 2026-11-02, so nothing is ported only to be ripped out and `.syl` is never
  unchecked): `dsl-named-arguments-anywhere` and `dsl-plain-word-names` first, with Lean still
  checking; then `semantics-rust-crate` (stage 1, `semantics`); then this epic (stage 2):
  `discourse`, stepping in spelling order (below), checked live against Lean; then
  `discourse-corpus-agreement` and `lean-retirement`.
- `lean-v3-monstrosity-adapt-name-the-source` went first because it removes what would otherwise
  be ported. (`lean-v3-sacrifice-performer-controls` was meant to go first too; it needs a new
  restrictor, so it follows.)
- The AST is frozen while they are built: a representation ticket that lands mid-port is done in
  both languages.
- The representation tickets need `lean-retirement` and are done afterwards, in `.syl` and Rust
  only; the open reference-resolution tickets (`discourse-*`) are part of this epic's work.
- `lowering` follows as `lowering-v3`.

## Spelling order

Settled (owner, 2026-11-02): argument order carries no meaning at a call site. A Template's or
Definition's spelling is one fixed string, so its arguments have one fixed order, the order its
spelling's holes print in; a call may write them in any order (`dsl-named-arguments-anywhere`, for
readability only). `discourse` steps a call's arguments in that fixed order. "<action> if
<condition>" (`doOnlyIf`) and "if <condition>, <action>" (`doIf`) are two spellings, so two
declarations, and the order is read from the spelling: `doIf`'s `postposed` flag and Lean's
order-recovering rules (`ReadOrder.fieldOrders`, the arms that step an intervening condition before
its action) are not ported. Declarations whose printed order differs from their parameter order
need their spelling (or an equivalent fixed order) before `discourse` relies on it.

Order is not timing, and this stays open for the design dialogue: in "<action> if <condition>" the
condition is checked before the action happens [CR#608.2c], so it reads the action as announced
(its targets, and its clause's re-mentions of earlier referents), not what the action creates or
moves (`ActionFamilies.postposedCounterConditionReadsTheUnmovedSpell`), while in `may(action, ifDid
= …)` the follow-up reads what the action moved. When a clause's outcomes become visible belongs to
the declaration's body, not to argument order. The carried-over tests for both must give the same
results.

Also open (2026-11-02 discussion, not rulings): whether Template bodies become opaque like
Definitions, every argument resolved once at the call site and each use checked against its own
slot (which would replace `discourse-parameter-argument-resolution`'s fix and Lean's
`inCaller`/`firstMention`/`laterMention`); and whether parameters with a default are always passed
by name and required ones never (a corpus audit found 118 required arguments passed by name and
1,300 optional ones positionally, about half of them verbs like `dealDamage`, `attack` and `block`
overloaded across valencies by defaults).

## Verification

`deckmaste_dsl` and the Lake generator parse the same `.syl` corpus to identical trees
(`dsl-parse-differential`, done). The Lean checker is the oracle for `discourse`, live while it is
built and in the tree for probing until `discourse-corpus-agreement` passes: the bindings and
fixture snapshot come from `lean-discourse-export`, keyed as that ticket says. `lean-retirement`
then deletes Lean.
