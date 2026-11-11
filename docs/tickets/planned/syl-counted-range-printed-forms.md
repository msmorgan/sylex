---
needs: []
---

# A range from one is written as its card prints it

## Why

`oneThrough` (`plugins/builtin/templates/quantity/oneThrough.syl`) is spelled "one to <n>", and no
caller prints that. Its canon callers print "one or two" (*Chandra's Pyrohelix*, *Electrolyze*,
*Pure Energy*), "one or both" (*Borrowed Malevolence*), "one, two, or three" (*Aerial Volley*)
and a die-roll row "1—9" (*Thunderwave*); keyword abilities use `oneThrough(1)` where fading,
impending and suspend print "a counter". `syl-canon-wording-defects` already asks how
`targets(oneThrough(2), isAnyTarget)` prints "one or two targets"; this ticket covers the
quantity itself everywhere else.

## Done when

- Each `oneThrough` call site is written with a declaration whose spelling is what its card or
  rule prints, and `oneThrough` is deleted or spells a form a caller prints.
- `cargo xtask dsl check` and `cargo xtask dsl fmt --check` pass.

## Pointers

- `plugins/builtin/templates/quantity/oneThrough.syl` and its callers
  (`grep -rn oneThrough plugins/`)
- `docs/tickets/planned/syl-canon-wording-defects.md` ("any target" open question)
