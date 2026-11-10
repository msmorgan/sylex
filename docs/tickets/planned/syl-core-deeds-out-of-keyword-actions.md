---
needs: [syl-explicit-deeds, syl-lean-doc-comments]
---

# Core-rules Deeds out of `keyword_action/`

## Why

`plugins/builtin/definitions/keyword_action/` holds 89 files, filed as keyword actions, but the
CR's keyword actions are the ones the CR lists [CR#701.1], and resolve is not among them:
resolving is the core procedure [CR#608]. `resolve.syl` is a core-rules Deed filed as a
keyword action, so the folder tells a reader something false about it.

Which declaration kind and folder a Deed lives in is settled by `syl-explicit-deeds`, which lands
first. `syl-lean-doc-comments` rewrites `resolve.syl`'s doc comment, and lands first so the file
is edited before it moves.

## Done when

- Every file in `keyword_action/` is checked against the keyword actions the CR's
  keyword-action section defines [CR#701.1], and the landing lists each file that is not a keyword action, its variant or its event.
- `resolve.syl`, and any other core-rules Deed found, moves to where `syl-explicit-deeds` settles
  such Deeds live; the landing lists the files moved.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Not in scope

- Keyword-action variants (`millCards`, `castWithoutPaying`, the `double…` family and the
  others): `syl-one-deed-per-keyword-action`.
- `resolve.syl`'s Lean-syntax doc comment: `syl-lean-doc-comments`.
- `devour.syl`, devour's event, which `syl-keyword-verb-events` deletes; `manifestPlacement.syl`,
  which `syl-dead-weight-sweep` deletes; `regeneration.syl`, regenerate's event, which
  `syl-explicit-deeds` settles.

## Pointers

`plugins/builtin/definitions/keyword_action/`, `docs/decisions/syl.md` §2 ("Paths are for
people").
