---
needs: [syl-single-face-card-call]
---

# "Each" and the referent it introduces

## Why

"Each" introduces a singular referent that verbs then patch to plural, so the same phrase is
read as one object in one place and a group in another, and each verb that takes it carries its
own fix.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Open questions

- Whether "each" introduces a plural referent, a distributive one read once per member, or stays
  singular with the patch moved to one place; and what that means for the verbs that patch it
  today.

## Done when

- "Each" is modeled as settled, the verbs' patches are gone, and every caller in `plugins/` reads
  as settled; the landing lists the verbs changed.
- `docs/decisions/semantics.md` states the rule.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/templates/noun/eachPlayer.syl` and the `each…` determiners under
`plugins/builtin/`, `docs/decisions/semantics.md` §2.4.
