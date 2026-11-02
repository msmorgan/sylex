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

Written order is for readability only; `semantics-rust-rewrite`'s `discourse` steps a call's
arguments in the order its declaration's spelling fixes.

## Change

- `deckmaste_dsl` parses calls whose named and positional arguments interleave. Positional
  arguments bind in written order, each to the next parameter not yet bound; a parameter bound
  twice, by name or by position, is an error. Interleaving is for readability only: argument order
  carries no meaning, since a template's parameter order is fixed by its spelling.
- `cargo xtask dsl fmt` keeps the written order, so an interleaved call round-trips.
- `sylGen` accepts the same calls: drop its "positional argument after named argument" refusal
  (`lean/SylGen/Resolve.lean`). It already sorts named arguments into binder order, so the Lean it
  emits, and Lean's checker, are unchanged.
- Semantics-v3 §4.4 ("Calls") says so.
- Tests in `deckmaste_dsl` for the interleaved forms (it only parses; the error is `sylGen`'s, tested
  in `negative.sh`); `cargo xtask lean check` passes on an interleaved fixture.
