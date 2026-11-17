---
needs: [syl-kind-sorts-and-position-unions]
specs: [kind-sorts-and-position-unions]
---

# Membership between sorts, and every modifier declared once at its kind

## Why

`PreModifier`, `PostModifier` and `Predicative` name a position, so a phrase is declared once per
position it stands in: 24 names stand at both `PostModifier` and `Predicative`, `hasColor` at
three sorts, and the copies drift (`attachment`, `inCombat` and `hasDesignation` have a different
signature per sort). "Isn't legendary" (*Yenna, Redtooth Regent*) has no term, because a
supertype word reaches `PreModifier` only. A coercion carries a word into one sort through a
wrapping primitive, one hop at a time, so each word needs a coercion per position and the wrapped
call is a second written form of the bare word. The spec replaces coercion with membership
between sorts and makes the positions sorts whose members are kinds of phrase; nothing in `cargo
xtask dsl check` knows a membership yet.

## Done when

- `cargo xtask dsl check` reads a `member` declaration, resolves a value by the spec's step 4,
  computes Reaches over members transitively, refuses a cycle, and enforces the spec's load rules:
  the shadowing of a member's coordinator, a nominal in `pre`, a membership's verb form, and the
  spelling-skeleton rule over Reaches. Each rule has a test that it refuses what the spec says it
  refuses. `coercion` is no declaration kind.
- `AdjectivePhrase`, `PrepositionPhrase` and `RelativeClause` are sorts, the memberships are the
  spec's §2 table, and the wrapping primitives it names are gone.
- Every declaration the spec's §4 lists under "one declaration at a kind", "deleted" and "words of
  the position sort" is what the spec says, and `plugins/` is rewritten where a name or a form
  changes. Each former caller of `not` at `PostModifier` writes what its card or rule prints. The
  noun and the verb `copy` have different names.
- *Yenna, Redtooth Regent* is authored: "isn't legendary".
- The landing lists every declaration still at a position sort that is neither its own word nor
  a position-bound Template, against the spec's "staying" table.
- The landing lists every call whose written form changed, by declaration, and runs the
  `syl-number-snapshot` diff and explains every flagged line.
- What stays true of the spec is in `syl.md`, `surface.md`, `spelling.md`, `semantics.md` and the
  glossary (spec §8), and `docs/tickets/planned/dsl-fmt-bundle-normalization.md` is deleted, since
  no wrapped form is left to normalize.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask`,
  `cargo test -p sylex_syl` and `cargo test -p sylex_spelling` pass.

## Not in scope

- The keyword parameters at `Predicate` and `hasQuality`: `syl-keyword-quality-union`.
- The "has …" wordings: `syl-have-verb-phrases`.
- The wordings the spec's "staying" table gives to the passive, lookback, zone-word and
  wording-defect tickets.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md`;
`crates/xtask/src/dsl_check/{coercion,nominal,schema,sort,spelling}.rs`, `crates/syl/`;
`plugins/builtin/{primitives,templates}/{preModifier,postModifier,predicative}/`,
`plugins/builtin/coercions/`; `docs/design/syl.md` §3, `docs/design/spelling.md` §7.
