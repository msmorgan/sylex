---
needs: [lean-generated-canon-modules]
---
# An untracked dump of every elaborated canon card and builtin declaration, from Lean

Standard constraints apply. `deckmaste_semantics_v3` and `deckmaste_lowering_v3` are going to be
deleted and rewritten (`semantics-rust-rewrite`). With them goes the only meaning-preservation
check: `cargo xtask expansions`. Lean needs its own, so that plugin rewrites can be compared
before and after once the Rust is gone, including the conversion in `plugins-v3-syl-conversion`.

## Change

- Add a `lake exe` target that is not one of the default `lean check` targets. It writes one
  record per canon card, keyed by printed name, holding the elaborated `Card` value.
- It also writes one record per builtin declaration that takes no arguments or has a defaulted
  invocation.
- Output goes to the gitignored `docs/evidence/lean-dumps/<label>/`, one sorted file. Each
  record is printed deterministically (`Repr`, or a printer written for the purpose where a spine
  type lacks one).
- Comparing two dumps is `diff`, plus a small rename-map option for when keys change.
- Sample builtin declarations that take parameters, the way `cargo xtask expansions` does today
  (`crates/deckmaste_semantics_v3/src/lower/snapshot.rs`). Write one record per
  `(bare_name, return_type, sample)`. Port the cases that matter:
  - every constructor of a finite enum parameter;
  - `Bool` both ways;
  - `Option` as both `none` and `some`;
  - `List` empty and with one element;
  - recursive constructors at depth 1, with each self-typed field filled by that type's base
    sample. The Rust tool silently skips these today (`Action.doIf`, `Action.delay`,
    `Amount.half` and others).

  A cap per declaration is allowed, but the summary reports what it leaves out. Without this,
  the `plugins-v3-syl-conversion` conversion would leave every builtin Template that no canon
  card uses unchecked.

## Acceptance

- Two runs on an unchanged tree are byte-identical.
- An edit that changes the meaning of one card (for example a cost pip) shows exactly that
  card's record as changed.
- An edit in a non-first `match` arm of a builtin Template, and in an arm on a recursive
  constructor, is reported as changed.
- Nothing the tool writes is tracked.
