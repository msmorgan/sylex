---
needs: [syl-kind-sorts-and-position-unions]
specs: [kind-sorts-and-position-unions]
---

# Membership, two resolution modes, one coordinator: every describing phrase at its kind

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

- `sylex_syl` parses `member(S): T`, `denotes(K): T` and a declaration's `at` list with `Self`,
  and `cargo xtask dsl check` resolves a card's term in surface mode and a body in body mode
  (spec §3), a word with an `at` list at the lowest of its sorts that holds its arms (spec §4),
  and enforces the spec's load rules: no cycle and no second path; a word at two sorts of one
  Reaches set in either mode; no `spelling` at `Test`; a `nominal(…)` with modifiers of its own
  in `pre` outside a coordination's arm; a declaration at a position sort that is neither a
  position-bound Template nor one of the spec's staying declarations, each named in the check
  with its ticket; a test in a card's term outside the named exceptions; and the
  spelling-skeleton rule over Reaches. Each rule has a test that it refuses what the spec says
  it refuses. `coercion` is no declaration kind.
- The sorts, memberships, coordinators and negations are the spec's §1 to §4, and every
  declaration at `PreModifier`, `PostModifier` and `Predicative` is what the spec's §5 says.
  `plugins/` is rewritten where a name or a form changes: each former caller of `not` at
  `PostModifier` writes what its card or rule prints, and *Solemnity*, *Clarion Conqueror*,
  *Damping Matrix* and *Grand Abolisher* coordinate their bare plurals as noun phrases.
- *Blessed Reversal*, *Tidal Flats*, *Beckoning Will-o'-Wisp* and *Triarch Stalker* write
  `attacking(…)` or `blocking(…)` where they call `inCombat`, and each new route the design
  opens that no existing call takes has one card authored or rewritten to take it: a supertype
  after a copula, "nonbasic" after a copula (*Molten Rain*), "not" before a preposition phrase
  (*Anafenza, the Foremost*), "aren't" (*Crippling Fear*), a participle with a dependent after a
  head, "face-up" before a head, "face down" after a copula.
- `docs/evidence/syl-kind-sorts/reframe_check.py` and `test_sort_check.py` are rerun for the
  spec's §5 as built, and the landing reports their collisions, unresolved bodies and
  coordinator calls with no common sort, which are none.
- The landing lists every call whose written form changed, by declaration, and every coordinator
  call whose resolved sort changed. It runs the `syl-number-snapshot` diff and explains every
  flagged line.
- What stays true of the spec is in `syl.md`, `surface.md`, `spelling.md`, `semantics.md` and the
  glossary (spec §10), and `docs/tickets/planned/dsl-fmt-bundle-normalization.md` is deleted,
  since no wrapped form is left to normalize.
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
`docs/evidence/syl-kind-sorts/{test-sort-check,reframe-check}.md` in the default workspace (the
computed placement of all 135 declarations, the two modes and the coordinator calls, with their
scripts; `move_or_stay.py`'s docstring describes the
dump of the checker's resolutions both scripts read, which a fresh workspace regenerates);
`crates/syl/src/{parse,check}.rs`, `crates/xtask/src/dsl_check/`, `crates/xtask/src/dsl_expand.rs`;
`plugins/builtin/{primitives,templates}/{preModifier,postModifier,predicative}/`,
`plugins/builtin/coercions/`; `docs/design/syl.md` §3, `docs/design/spelling.md` §7.
