---
needs: [semantics-rust-rewrite]
---
# Named arguments may stand between positional ones

A call may put a named argument anywhere among its positional ones, so the `.syl` can follow the
printed order of the text it encodes. An intervening "if" is printed between the trigger event and
the effect ("When this Aura enters, if it's on the battlefield, …"), and should be written there:

```syl
when(
  does(actor(thisAura), enter),
  if = is(it, in(battlefield)),
  sequentially([...]))
```

Today `sylGen` refuses a positional argument after a named one (`lean/SylGen/Resolve.lean`,
"positional argument after named argument"); it goes with Lean in `semantics-rust-rewrite`.

## Change

- `deckmaste_dsl` parses and Rust `semantics` resolves calls whose named and positional arguments
  interleave. Positional arguments still bind in order, to the parameters not named; naming a
  parameter twice, or naming one a positional argument already filled, is an error.
- `cargo xtask dsl fmt` keeps the written order, so an interleaved call round-trips.
- Semantics-v3 §4.4 ("Calls") says so.
- Reorder the canon and builtin calls whose named arguments have a printed position, the
  intervening-"if" triggers first (`when`, `whenever`, `at` with `if =`). Reordering does not
  change any expansion: check that `semantics` expands every touched file identically before and
  after.
- Tests in `deckmaste_dsl` and `semantics` for the interleaved forms and the two errors.
