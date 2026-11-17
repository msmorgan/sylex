---
needs: [syl-kind-sorts-and-position-unions]
specs: [kind-sorts-and-position-unions]
---

# Membership between sorts, a sort for the unspelled tests, and every describing phrase at its kind

## Why

`PreModifier`, `PostModifier` and `Predicative` name a position, so a phrase is declared once per
position it stands in: 24 names stand at both `PostModifier` and `Predicative`, `hasColor` at
three sorts, and the copies drift (`attachment`, `inCombat` and `hasDesignation` have a different
signature per sort). "Isn't legendary" (*Yenna, Redtooth Regent*) has no term, because a
supertype word reaches `PreModifier` only, and *Balance*'s "lands controlled by the player who
controls the fewest" cannot be written. A coercion carries a word into one sort through a
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
  `Test`; a parameter typed by a kind; a nominal or a coordinated nominal in `pre`; and the
  spelling-skeleton rule over Reaches. Each rule has a test that it refuses what the spec says it
  refuses. `coercion` is no declaration kind.
- The sorts and memberships are the spec's §1 and §2, and every declaration at `PreModifier`,
  `PostModifier` and `Predicative` is what the spec's §4 says: moved, declared once at `Test`,
  deleted, renamed, or staying. `plugins/` is rewritten where a name or a form changes, and each
  former caller of `not` at `PostModifier` writes what its card or rule prints.
- *Yenna, Redtooth Regent* and *Balance* are authored, and *Blessed Reversal*, *Tidal Flats*,
  *Beckoning Will-o'-Wisp* and *Triarch Stalker* write `attacking(…)` or `blocking(…)` where they
  call `inCombat`.
- The landing lists every declaration still at a position sort that is neither its own word nor
  a position-bound Template, against the spec's "stays" table, and every call whose written form
  changed, by declaration. It runs the `syl-number-snapshot` diff and explains every flagged line.
- What stays true of the spec is in `syl.md`, `surface.md`, `spelling.md`, `semantics.md` and the
  glossary (spec §8), and `docs/tickets/planned/dsl-fmt-bundle-normalization.md` is deleted, since
  no wrapped form is left to normalize.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` (the `dsl expand`
  tests that assert a coercion's wrap among them), `cargo test -p sylex_syl` and
  `cargo test -p sylex_spelling` pass.

## Not in scope

- The wordings cards print where they call a test bare: `syl-spelled-test-phrases`.
- The keyword parameters at `Predicate` and `hasQuality`: `syl-keyword-quality-union`.
- The "has …" wordings: `syl-have-verb-phrases`.
- Replacing a fixed participle with its verb: `syl-passive-participle-and-trigger-events`.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md`;
`docs/evidence/syl-kind-sorts/test-sort-check.md` (the computed placement of all 135
declarations, with the script that produced it);
`crates/syl/src/{parse,check}.rs`, `crates/xtask/src/dsl_check/`, `crates/xtask/src/dsl_expand.rs`;
`plugins/builtin/{primitives,templates}/{preModifier,postModifier,predicative}/`,
`plugins/builtin/coercions/`; `docs/design/syl.md` §3, `docs/design/spelling.md` §7.
