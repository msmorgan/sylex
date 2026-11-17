---
needs: [syl-kind-sorts-and-position-unions]
specs: [kind-sorts-and-position-unions]
---

# Unions of kind sorts, and every modifier declared once at its kind

## Why

`PreModifier`, `PostModifier` and `Predicative` name a position, so a phrase is declared once per
position it stands in: 24 names stand at both `PostModifier` and `Predicative`, `hasColor` at
three sorts, and the copies drift (`attachment`, `inCombat` and `hasDesignation` have a different
signature per sort). "Isn't legendary" (*Yenna, Redtooth Regent*) has no term, because a
supertype word reaches `PreModifier` only. The spec makes the positions unions of kinds of
phrase; nothing in `cargo xtask dsl check` knows a union yet.

## Done when

- `cargo xtask dsl check` reads a `union` declaration, resolves a value at a union by the spec's
  step 5, computes Reaches over members, and enforces the spec's load rules: what a union may
  declare, the shadowing of a member's coordinator, a nominal in `pre`, the member's verb form,
  and the spelling-skeleton rule over a union's Reaches. Each rule has a test that it refuses what
  the spec says it refuses.
- `AdjectivePhrase`, `PrepositionPhrase` and `RelativeClause` are sorts; `PreModifier`,
  `PostModifier`, `Predicative` and `Quality` are unions with the spec's members; and the six
  coercions of spec §4 are the coercions that exist.
- Every declaration the spec's §5 lists under "one declaration at a kind", "deleted as a
  duplicate", "words of the union", "identity wrappers deleted" and "unspelled tests" is what the
  spec says, and `plugins/` is rewritten where a name or a form changes. `not` at `PostModifier`
  is gone, each of its callers writing what its card or rule prints.
- *Yenna, Redtooth Regent* and *Balance* are authored: "isn't legendary" and "lands controlled by
  the player who controls the fewest".
- The landing lists every call whose written form changed, by declaration, and runs the
  `syl-number-snapshot` diff and explains every flagged line.
- What stays true of the spec is in `syl.md`, `surface.md`, `spelling.md`, `semantics.md` and the
  glossary (spec §8).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo test -p sylex_spelling` pass.

## Not in scope

- The keyword parameters at `Predicate`: `syl-keyword-quality-union`.
- The "has …" wordings: `syl-have-verb-phrases`.
- The wordings spec §5 gives to the passive, lookback, zone-word and wording-defect tickets.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md`;
`crates/xtask/src/dsl_check/{coercion,nominal,schema,sort,spelling}.rs`;
`plugins/builtin/{primitives,templates}/{preModifier,postModifier,predicative,predicate}/`,
`plugins/builtin/coercions/`; `docs/design/syl.md` §3, `docs/design/spelling.md` §7.
