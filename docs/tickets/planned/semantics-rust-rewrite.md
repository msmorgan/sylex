---
needs: [semantics-rust-crate, lean-v3-monstrosity-adapt-name-the-source, lean-proofs-spec-catalog, idris-retirement]
---
# [design] Rewrite the Rust semantics pipeline: `semantics`, `discourse`, `lowering`

Standard constraints apply. This is an epic, and needs a design dialogue before it is split into
tickets.

`plugins-v3-syl-conversion` deletes `deckmaste_semantics_v3`, `deckmaste_lowering_v3` and
`xtask expansions` when it converts the files. This epic starts from that state and builds three
crates:

- **`semantics`** resolves names by expected sort (§4.5) and expands Templates and Definitions over the `.syl` syntax tree
  that `deckmaste_dsl` parses (`dsl-parser`; grammar in `docs/decisions/semantics-v3.md` §4.4). It does no reference resolution.
- **`discourse`** implements the DRT referent resolution of `docs/decisions/semantics-v3.md` §1
  and §3 in Rust. Lean is never in the hot path of card compilation, so this is a
  reimplementation, not a reader of Lean output. The deleted `deckmaste_lowering_v3` had part of it; recover what is useful from jj history.
- **`lowering`** translates the resolved AST to the `deckmaste_core` representation. Its seams
  are as already designed in `lowering-v3`: regions, `ownScope` to `Action::Composite`, the
  action-clause spine, and so on.

## Sequencing (owner, 2026-11-02)

This replaces the 2026-10-28 ruling, which put every remaining `lean-v3-*` ticket first so that
the Lean workbench's semantics would settle before Rust reimplemented them. The workbench's
architecture (the action-clause spine, the Definition/Template split, DRT reference resolution)
is settled; what remained was mostly AST breadth, which done in Lean first is written twice.

- Stages (owner, 2026-11-02, so nothing is ported only to be ripped out and `.syl` is never
  unchecked): `dsl-named-arguments-anywhere` and `dsl-plain-word-names` first, with Lean still
  checking; then `semantics-rust-crate` (stage 1, `semantics`); then this ticket (stage 2):
  `discourse`, stepping in spelling order (below), checked live against Lean; then Lean is
  deleted.
- `lean-v3-monstrosity-adapt-name-the-source` went first because it removes what would otherwise
  be ported. (`lean-v3-sacrifice-performer-controls` was meant to go first too; it needs a new
  restrictor, so it follows.)
- The AST is frozen while they are built: a representation ticket that lands mid-port is done in
  both languages.
- The representation tickets and the open reference-resolution tickets need this epic and are
  done afterwards, in `.syl` and Rust only.
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

## Retiring Lean

This epic ends by deleting `lean/` and `sylGen` in full, after `idris-retirement` has deleted
`idris/` (the older workbench goes first; owner, 2026-11-02), and once:

- per card, `discourse`'s bindings agree with the Lean oracle (below) across the whole corpus;
- the exported fixture snapshot (below) passes;
- `lean-proofs-spec-catalog`'s catalog exists as native Rust tests. After retirement it and the
  canon corpus are the only record of what must and must not check.

Settled (owner, 2026-11-02): no Lean mirror of the DSL is kept. An earlier ruling that day kept
the AST declarations and `sylGen` as a mirror; it is withdrawn, because a mirror would keep
the DSL bound to Lean: its keywords (the source of the trailing-underscore names) and an AST that
records no written order.

The snapshot is deleted with Lean, not maintained: its expected outputs are frozen then, and the
first AST change after the port would break fixtures nothing can regenerate.

## Verification

`deckmaste_dsl` and the Lake generator parse the same `.syl` corpus to identical trees;
`dsl-parse-differential` gates this before the epic starts.

The Lean checker is the oracle for `discourse`, live while `discourse` is built (it stays in the
tree for probing until the gates below pass). Per card, Rust's resolved bindings (first and later
mentions, `D′`, `AC>R1` choices) must match the bindings Lean's checker produces. Lean exports
them as a development tool, outside the compile path. A mention and its antecedent are keyed by
their expanded core AST path, a sequence of (constructor, field, list index) from the card root,
plus a role tag for what that node introduces (`Intro(k)` for its k-th referent, `Outcome(sort)`,
`MovedSuccessor`, `LoopAggregate`, …), not by surface parameter names, argument positions or
referent ids: list elements, a Definition parameter substituted at several sites, and antecedents
that are no argument (created tokens, outcomes, piles, mana letters, zone-move successors) all
need it, and ids depend on stepping order. Entries the spec catalog marks FLAG DEFECT
(`discourse-parameter-argument-resolution`) are expected to differ, `discourse` fixing the defect.
The design dialogue settles the export format and how per-card agreement is gated.

The same export also evaluates the closed `*.check … = …` equations in `lean/Semantics/Proofs/`
into an (input, expected) fixture snapshot that Rust runs as one data-driven test. A fixture
compares accepted against refused and the set of refusal kinds, never the exact refusal list:
matching Lean's multiplicity and order would carry its plumbing into Rust. Keep the exporter
disposable: it handles the plain equations over the public entry points (`Card.check`,
`Ability.check`, `Action.check`, the binding results) and emits the names of every other theorem
(statements with `∧`, `≠`, `.all` or `let`, and those over checker internals such as `stepAction`
or `causedEvents`) as a checklist for `lean-proofs-spec-catalog`.
