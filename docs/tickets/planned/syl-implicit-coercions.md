---
needs: [syl-primitive-constructors]
---
# Declared implicit wrapping in `.syl`

Standard constraints apply. After `syl-primitive-constructors`, `.syl` has no implicit
conversions: a value of one sort reaches a position of another only through a wrapper written at
the call site. Colors in a mana cost, integers at `Amount` and in mana costs, a Definition's name as
its keyword label, keyword term or Deed are all wrapped by hand (`Option` alone stays implicit).

Design how a declaration says that a wrapper may be left implicit (for example a field on the
wrapping primitive), so that `cost = [1, blue, blue]` reads again with the rule declared once in
`.syl`, not special-cased in the resolver. Settle with the owner, before implementing:

- which wrappers may be implicit, and whether implicit wraps may chain (`Color` to
  `ColorOrColorless` to a mana symbol);
- how this sits with §4.4's "no coercions" and with expected-sort resolution (§4.5), so that a
  resolution stays deterministic, needs no backtracking, and has one reading;
- which of the hand-written wraps (colors, integers, Definition names) become implicit,
  each declared once in `.syl`.

## Tests

- The canon corpus, respelled without the explicit wrappers, expands identically.
- An ambiguous implicit wrap (two wrappers reaching one sort) is refused at load.
