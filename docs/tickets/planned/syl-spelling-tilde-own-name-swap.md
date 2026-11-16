---
needs: [cleanup-step6-surface]
---

# Spelling notation swaps `~`/`~~` to own-name tokens and `#` to affix ties

## Why

Community Magic shorthand writes `~` for a card's own name, whereas `docs/design/spelling.md` §7
currently assigns `~` to bound affix ties (`non~$p`, `$n~th`) and `#` / `##` to `ownName` and
`shortenedName` alongside `#<join>` list joins. Swapping them puts `~` and `~~` on full and
shortened own-name spellings and `#` on affix ties (`non#$p`, `$n#th`), unifying `#` around Hole
attachment (`affix#$hole`, `$hole#affix`, `$list#<join>`).

## Design

- `ownName` has `spelling = "~"` and `shortenedName` has `spelling = "~~"`, each standing alone as
  the whole `spelling` of its declaration.
- A bound prefix or suffix attaches to a Hole with `#` (`non#$p`, `$n#th`); list joins keep
  `$name#<join>`, distinguished from a suffix tie by `<` after `#`.

## Done when

- `sylex_spelling` parses `"~"` and `"~~"` as `Spelling::OwnName(OwnName::Full | OwnName::Shortened, _)`
  and `#` (not followed by `<`) on a Hole as the affix tie, refusing `~` inside parts.
- Every affix-tied `spelling` in `plugins/builtin/` and `ownName` / `shortenedName` are rewritten
  to the swapped marks, and `docs/design/spelling.md` §7 is updated to match.
- `cargo test -p sylex_spelling`, `cargo test -p xtask`, `cargo clippy -p sylex_spelling -p xtask --tests -- -D warnings`,
  `cargo xtask dsl check` and `cargo xtask dsl fmt --check` pass.

## Pointers

`crates/spelling/src/{lib,notation}.rs`, `crates/xtask/src/dsl_check/{spelling,self_name}.rs`,
`plugins/builtin/primitives/pronoun/ownName.syl`,
`plugins/builtin/templates/pronoun/shortenedName.syl`, `docs/design/spelling.md` §7.
