---
needs: [lean-build-speed]
---
# An untracked dump of every elaborated canon card and builtin declaration, from Lean

Standard constraints apply. `deckmaste_semantics_v3` and `deckmaste_lowering_v3` are going to be
deleted and rewritten (`semantics-rust-rewrite`). With them goes the only meaning-preservation
check: `cargo xtask expansions`. Lean needs its own, so that plugin rewrites can be compared
before and after once the Rust is gone, including the conversion in `plugins-v3-data-language`.

## Change

- Add a `lake exe` target that is not one of the default `lean check` targets. It writes one
  record per canon card, keyed by printed name, holding the elaborated `Card` value.
- It also writes one record per builtin declaration that takes no arguments or has a defaulted
  invocation.
- Output goes to the gitignored `docs/evidence/lean-dumps/<label>/`, one sorted file. Each
  record is printed deterministically (`Repr`, or a printer written for the purpose where a spine
  type lacks one).
- Comparing two dumps is `diff`, plus a small rename-map option for when keys change.
- Builtin templates with parameters are covered only through the cards that use them. Say so in
  the tool's doc comment; the Rust baseline's per-parameter sampling is not ported.

## Acceptance

- Two runs on an unchanged tree are byte-identical.
- An edit that changes the meaning of one card (for example a cost pip) shows exactly that
  card's record as changed.
- Nothing the tool writes is tracked.
