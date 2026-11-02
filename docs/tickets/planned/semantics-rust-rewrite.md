---
needs: [dsl-parser, dsl-parse-differential, plugins-v3-data-language, lean-v3-monstrosity-adapt-name-the-source, lean-v3-sacrifice-performer-controls]
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

- `semantics` and `discourse` start once `dsl-parse-differential` lands, against the current AST,
  after only the two tickets that remove or simplify what would otherwise be ported
  (`lean-v3-monstrosity-adapt-name-the-source`, `lean-v3-sacrifice-performer-controls`).
- The AST is frozen while they are built: a representation ticket that lands mid-port is done in
  both languages.
- The representation tickets and the open reference-resolution tickets need this epic and are
  done afterwards, in `.syl` and Rust only.
- `lowering` follows as `lowering-v3`.

## Retiring Lean

This epic ends by deleting `lean/` and `sylGen`, once:

- per card, `discourse`'s bindings agree with the Lean oracle (below) across the whole corpus;
- the Lean proof suites are carried over as Rust tests, refusals included: after retirement they
  are the only record of what must and must not check, and nothing else oracles the tickets that
  follow.

## Verification

`deckmaste_dsl` and the Lake generator parse the same `.syl` corpus to identical trees;
`dsl-parse-differential` gates this before the epic starts.

The Lean checker is the oracle for `discourse`. Per card, Rust's resolved bindings (first and
later mentions, `D′`, `AC>R1` choices) must match the bindings Lean's checker produces. Lean can
export them as a development tool, outside the compile path. The design dialogue settles that
export and how per-card agreement is gated.
