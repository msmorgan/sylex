---
needs: []
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

This ticket only makes the language accept it; `syl-printed-order` reorders the corpus, and
`semantics-rust-rewrite`'s `discourse` steps arguments in written order.

## Change

- `deckmaste_dsl` parses calls whose named and positional arguments interleave. Positional
  arguments still bind in order, to the parameters not named; naming a parameter twice, or naming
  one a positional argument already filled, is an error.
- `cargo xtask dsl fmt` keeps the written order, so an interleaved call round-trips.
- `sylGen` accepts the same calls: drop its "positional argument after named argument" refusal
  (`lean/SylGen/Resolve.lean`). It already sorts named arguments into binder order, so the Lean it
  emits, and Lean's checker, are unchanged; Lean stays the oracle for reordered `.syl`.
- Semantics-v3 §4.4 ("Calls") says so.
- Tests in `deckmaste_dsl` for the interleaved forms and the two errors; `cargo xtask lean check`
  passes on an interleaved fixture.
