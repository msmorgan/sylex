---
needs: [syl-single-face-card-call, cleanup-step6-spine, syl-number-snapshot, syl-nominal-descriptions]
---

# "Each" and the referent it introduces

## Why

"Each" introduces a singular referent that verbs then patch to plural, so the same phrase is
read as one object in one place and a group in another, and each verb that takes it carries its
own fix.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

Written against the spine (`cleanup-step6-spine`), after `syl-nominal-descriptions`, which
retypes the `each…` determiners and rewrites their callers' descriptions.

## Design

- "Each" is distributive. Inside its nuclear scope its referent is singular: "Each creature deals
  damage to itself equal to its power" (*Wave of Reckoning*), and "their" in "on each of their
  turns" (*Rites of Flourishing*) is singular.
- On leaving that scope, its domain and the indefinites introduced inside it are exported as
  plural: "Destroy each creature with mana value 3 or less. They can't be regenerated."
  (*Consume the Meek*); "Each player puts a vow counter on a creature they control … Each of
  those creatures …" (*Promise of Loyalty*).

## Done when

- "Each" is modeled as the Design states, the verbs' patches are gone, and every caller in
  `plugins/` is written to match; the landing lists the verbs changed.
- Which referent each anaphor resolves to is not checkable until reference resolution exists in
  step 8; the landing shows the readings of the cards above for the owner to review.
- `docs/decisions/semantics.md` states the rule.
- Declaration order stays the printed order (`syl.md` §5).
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/templates/noun/eachPlayer.syl` and the `each…` determiners under
`plugins/builtin/`, `docs/decisions/semantics.md` §2.4.
