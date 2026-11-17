---
needs: [syl-kind-sorts-and-position-unions]
specs: [kind-sorts-and-position-unions]
---

# Membership between sorts, a sort for the unspelled tests, and every describing phrase at its kind

## Why

`PreModifier`, `PostModifier` and `Predicative` name a position, so a phrase is declared once per
position it stands in: 24 names stand at both `PostModifier` and `Predicative`, `hasColor` at
three sorts, and the copies drift (`attachment`, `inCombat` and `hasDesignation` have a different
signature per sort). A supertype word reaches `PreModifier` only, so "isn't legendary" is
written by an ability Template of its own, and "controlled by $who" stands only at `Predicative`,
so it cannot follow a head. A coercion carries a word into one sort through a
wrapping primitive, one hop at a time, so the wrapped call is a second written form of the bare
word. The unspelled tests the wordings are written over are copied per position too. The spec
replaces coercion with membership between sorts, makes the positions sorts whose members are
kinds of phrase, and declares each test once; nothing in `sylex_syl` or `cargo xtask dsl check`
knows a membership yet.

## Done when

- `sylex_syl` parses `member(S): T` and `expands(K): T`, and `cargo xtask dsl check` resolves a
  value by the spec's step 4, computes Reaches over members transitively, and enforces the spec's
  load rules: no cycle and no second path; a word at two sorts of one Reaches set, with the
  position sort's own words excepted; no `spelling` at `Test`; a Template body at its kind or at
  `Test`; a parameter typed by a kind that `expands`, and a word at such a kind that a position
  sort it is a member of declares itself; a `nominal(…)` with modifiers of its own, or
  a coordinated one, in `pre` outside a coordination's arm; a declaration at a position sort that
  is neither its own word, a position-bound Template, nor one of the spec's staying declarations,
  each named in the check with its ticket; and the spelling-skeleton rule over Reaches. Each rule has a test that it refuses what the spec says it
  refuses. `coercion` is no declaration kind.
- The sorts and memberships are the spec's §1 and §2, and every declaration at `PreModifier`,
  `PostModifier` and `Predicative` is what the spec's §4 says: moved, declared once at `Test`,
  deleted, renamed, or staying. `plugins/` is rewritten where a name or a form changes, and each
  former caller of `not` at `PostModifier` writes what its card or rule prints.
- *Blessed Reversal*, *Tidal Flats*, *Beckoning Will-o'-Wisp* and *Triarch Stalker* write
  `attacking(…)` or `blocking(…)` where they call `inCombat`, and each new route a membership
  opens that no existing call takes (a supertype after a copula, a participle with a dependent
  after a head, "face down" after a copula) has one card authored or rewritten to take it.
- `docs/evidence/syl-kind-sorts/test_sort_check.py` is rerun for the spec's §4 as built, and the
  landing reports its collisions and unresolved bodies, which are none.
- The landing lists every call whose written form changed, by declaration. It runs the `syl-number-snapshot` diff and explains every flagged line.
- What stays true of the spec is in `syl.md`, `surface.md`, `spelling.md`, `semantics.md` and the
  glossary (spec §8), and `docs/tickets/planned/dsl-fmt-bundle-normalization.md` is deleted, since
  no wrapped form is left to normalize.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` (the `dsl expand`
  tests that assert a coercion's wrap and the coercion tests in `dsl_check.rs` among them, each
  rewritten for membership or retired with its reason), `cargo test -p sylex_syl` and
  `cargo test -p sylex_spelling` pass.

## Not in scope

- The wordings cards print where they call a test bare: `syl-spelled-test-phrases`.
- The keyword parameters at `Predicate` and `hasQuality`: `syl-keyword-quality-union`.
- The "has …" wordings: `syl-have-verb-phrases`.
- Replacing a fixed participle with its verb: `syl-passive-participle-and-trigger-events`.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md`;
`docs/evidence/syl-kind-sorts/test-sort-check.md` in the default workspace (the computed
placement of all 135 declarations, with its script; `move_or_stay.py`'s docstring describes the
dump of the checker's resolutions both scripts read, which a fresh workspace regenerates);
`crates/syl/src/{parse,check}.rs`, `crates/xtask/src/dsl_check/`, `crates/xtask/src/dsl_expand.rs`;
`plugins/builtin/{primitives,templates}/{preModifier,postModifier,predicative}/`,
`plugins/builtin/coercions/`; `docs/design/syl.md` §3, `docs/design/spelling.md` §7.
