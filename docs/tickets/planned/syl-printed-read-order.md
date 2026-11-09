---
needs: [lean-retirement, syl-argument-naming, syl-sort-prefixed-primitive-names, syl-at-turn-point, syl-spelling-optional-segments, syl-enlist-optional-attack-cost, syl-single-face-card-call]
---
# Make every declaration's read order its printed order, and drop `doIf`'s `postposed`

## Why

Read order is printed order (`docs/decisions/syl.md` §5): where a declaration's printed order
differs from its parameter order, the parameters are reordered to match. Several `.syl`
declarations still differ, and `doIf` carries a `postposed: Bool` flag that says in a boolean what
the printed order should say. Reordering puts some required parameters after optional ones;
under `syl-argument-naming`'s rule a positional argument binds only required parameters, so those
call sites need no naming.

The same sites are rewritten by four other tickets, which land first:
`syl-sort-prefixed-primitive-names` renames the `actionPay`, `actionRemoveCounters` and
`actionVote` calls inside the `pay`, `removeCounters` and `vote` declarations (10 calls) that this
ticket reorders; `syl-at-turn-point` changes `at.syl`'s event parameter;
`syl-spelling-optional-segments` merges the `preventAll`, `preventAllBy` and `preventNext`
Templates whose `preventDamage` calls this ticket reorders (3 files); and
`syl-enlist-optional-attack-cost` gives `gatedBy` and `staticSpecGatedBy` their offered form
(2 files).

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Done when

- Every declaration in `plugins/builtin/` whose printed order differs from its parameter order is
  reordered to match, with its call sites in `plugins/` updated. Cases found so far (re-survey
  when claimed): the trigger templates `when`, `whenever` and `at` (the intervening `if` is
  printed before the action), `gatedBy` and `staticSpecGatedBy` (the cost is printed last), and
  the `preventDamage`, `removeCounters`, `vote` and `pay` families.
- `doIf` has no `postposed` parameter; `doOnlyIf` ("<action> if <condition>") says the printed
  order itself, and what the flag changed for reference resolution is kept by that order. That
  equivalence is not checkable until reference resolution exists; the landing lists every former
  `postposed` call with its new form.
- The reordered corpus means the same: a one-off script, kept in `docs/evidence/`, checks that
  every call in `plugins/` binds the same parameter-to-value map before and after (apart from
  `postposed`), and the landing reports the declaration and call counts it compared.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `docs/decisions/syl.md` §5
- `plugins/builtin/templates/ability/when.syl`, `whenever.syl`, `at.syl`,
  `plugins/builtin/templates/static/gatedBy.syl`,
  `plugins/builtin/primitives/static/staticSpecGatedBy.syl`,
  `plugins/builtin/primitives/action/doIf.syl`, `plugins/builtin/templates/action/doOnlyIf.syl`
