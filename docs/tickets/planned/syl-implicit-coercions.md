---
needs: [syl-primitive-constructors]
---

# Declared implicit wrapping in `.syl`

## Why

After `syl-primitive-constructors`, `.syl` has no implicit conversions besides `Option`:
a value of one sort reaches a position of another only through a wrapper written at the call
site. Colors and integers in a mana cost, integers at `Amount`, and a Definition's name as its
keyword term are all wrapped by hand, so `cost = [1, blue, blue]` no longer reads as printed.
`syl.md` §2 leaves room for coercions to return deliberately once the language is simple enough.

## Open questions

- Which wrappers may be left implicit.
- Whether implicit wraps may chain (`Color` to `ColorOrColorless` to a mana symbol).
- How they sit with §2's "no conversions" and §3's resolution by expected sort, so that
  resolution stays deterministic, needs no backtracking, and has one reading.

## Done when

- A declaration states that its wrapper may be left implicit (for example a field on the wrapping
  primitive), once in `.syl`; the resolver has no special case for any sort.
- The settled wrappers are implicit, and the canon corpus respelled without them means
  the same as before.
- An ambiguous implicit wrap (two wrappers reaching one sort) is refused at load, with a test.
- `docs/decisions/syl.md` §2 and §3 state the rule.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/decisions/syl.md` §2 and §3, `crates/syl/`, `syl-primitive-constructors`.
