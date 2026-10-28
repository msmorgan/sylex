---
needs: [plugins-v3-expansion-baseline-recursive-samples]
---
# First and later mentions follow constructor order, whatever order a declaration takes its parameters in

Standard constraints apply.

## The defect

`docs/decisions/semantics-v3.md` §1 makes constructor argument order textual order: the DRS
threads left to right through constructor arguments.

Authoring disagrees. `bindParameters` and `rewriteMentions` (`lean/Semantics/Authoring.lean`
around `:379` and `:583`) decide whether an occurrence of a NounPhrase parameter is its
`firstMention` or a `laterMention`. They do so by walking the declaration body before calls
are expanded ("Keep application order and authored calls intact", `:377`). So when an authored
callee orders its parameters differently from the constructor underneath, mentions are
numbered in the callee's order, not the constructor's.

The Rust lowering builds the same ids (`<decl>.<param>`, `crates/deckmaste_semantics_v3/src/lower/mod.rs`
around `:1037`) and has to agree.

`plugins-v3-primitive-template-boundary` wrote a workaround rule into §4.5 ("Names and
constructor fields"): a primitive's parameter order must preserve the constructor's. Twelve
primitives already break it:

- `conjunction`, `choose`, `specialAction`, `delay`, `triggerReflexively`, `attachment` and
  `ofChosen` were reordered in that ticket.
- `spell`, `addTurnPart`, `moveCounters` and both `nthOccurrence`s were already out of order.

The rule also says nothing about Templates, which have the same hazard. No current caller is
affected, because every caller passes `none` in the moved slots.

Failure scenario: a Template passes one NounPhrase parameter both in `conjunction`'s `parts`
and, through `some p`, as its `subject`. The first mention then lands in `parts`, against
reading order.

## Ruling (owner, 2026-10-28)

Fix the binding, not the signatures. A declaration's parameter order carries no meaning:
primitive and Template signatures may follow the printed phrase. Mentions are numbered in the
order the constructor arguments they reach are read.

## Change

- Number mentions after authored declarations (primitives and Templates) are expanded down to
  constructor applications. Ordinary helpers stay unexpanded, preserving the boundary the
  `:377` comment protects (`acceptsParameterIdentity`).
  - Primitives are pure permutations of their constructor, so expanding them is exact.
  - A Template parameter may reach several constructor positions, or pass through a `match`.
    Number each occurrence where it lands after expansion.
- Mirror the change in the Rust lowering.
- Remove the parameter-order rule from §4.5, and record the ruling above in §4.5 or beside
  §1's reading-order rule.

## Acceptance

- New Lean pin and Rust test: a Template that passes the same NounPhrase as `conjunction`'s
  `parts` and its `subject` gives the subject the `firstMention`. Add the same check for one
  reordered Template.
- Builtin and canon expansion baselines show 0 differences, since no current caller depends on
  the order.
- Gates: `cargo xtask lean check`. Run `cargo test` and clippy for `deckmaste_semantics_v3`,
  `deckmaste_lowering_v3` and `xtask`.
