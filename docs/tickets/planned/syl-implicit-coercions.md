---
needs: [syl-primitive-constructors]
---
# Declared implicit wrapping in `.syl`

Standard constraints apply. After `syl-primitive-constructors`, a value of one sort reaches a
position of another only through a wrapper written at the call site: a color in a mana cost is
wrapped in the mana-symbol primitive by hand, because the resolver's old rule that color words are
valid at `Color`, `ColorOrColorless` and `ManaSymbol` is gone.

Design how a declaration says that a wrapper may be left implicit (for example a field on the
wrapping primitive), so that `cost = [1, blue, blue]` reads again with the rule declared once in
`.syl`, not special-cased in the resolver. Settle with the owner, before implementing:

- which wrappers may be implicit, and whether implicit wraps may chain (`Color` to
  `ColorOrColorless` to a mana symbol);
- how this sits with §4.4's "no coercions" and with expected-sort resolution (§4.5), so that a
  resolution stays deterministic, needs no backtracking, and has one reading;
- whether integer literals at `Amount` and at a mana symbol become declared wraps of the same kind
  or stay direct atoms.

## Tests

- The canon corpus, respelled without the explicit wrappers, expands identically.
- An ambiguous implicit wrap (two wrappers reaching one sort) is refused at load.
