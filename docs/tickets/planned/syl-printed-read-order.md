---
needs: [lean-retirement, syl-argument-naming, syl-sort-prefixed-primitive-names, syl-at-turn-point, syl-spelling-optional-segments, syl-single-face-card-call]
---
# Make every declaration's read order its printed order, and drop `doIf`'s `postposed`

## Why

Read order is printed order (`docs/design/syl.md` §5): where a declaration's printed order
differs from its parameter order, the parameters are reordered to match. Several `.syl`
declarations still differ, and `doIf` carries a `postposed: Bool` flag that says in a boolean what
the printed order should say. Arguments are written in declaration order, and `cargo xtask dsl
check` refuses any other (`syl.md` §2), so where a declaration's parameter order contradicts its
own spelling every call is forced to write its arguments against printed order. Reordering puts
some required parameters after optional ones; a positional argument binds only required
parameters, so those call sites need no naming.

The same sites are rewritten by another ticket, which lands first:
`syl-sort-prefixed-primitive-names` folds `pay` and renames `castVote` inside the
`pay` and `vote` declarations that this ticket reorders.
`syl-enlist-optional-attack-cost` gives `gatedBy` and `staticSpecGatedBy` their offered form after
this ticket reorders them.

## Done when

- Every declaration in `plugins/builtin/` whose printed order differs from its parameter order is
  reordered to match, with its call sites in `plugins/` updated. The survey is a one-off script,
  kept in `docs/evidence/`, that compares each declaration's parameter order with the order of
  its `spelling`'s Holes, and, for a declaration with no `spelling`, with the Holes of the wording
  its doc comment gives; it is re-run when the ticket is claimed. Cases found so far:
  - with no `spelling`: `gatedBy` and `staticSpecGatedBy` (the cost is printed last), and the
    `vote` and `pay` families;
  - whose `spelling` prints a Hole before an earlier parameter's, 13 of the 15 the script finds:
    `markDamage(subject, amount)` "$amount damage is marked on $subject", `headed(head,
    subject)` "$subject as a $head", `abilityGrantSelected(subject, source, selection)` "$subject
    has $selection of $source", `devourEntry(quality, head, amount)`, `delayUnlessPaid(event,
    action, player, cost)` "$action at $event unless $player pays $cost …", `doUnless(action,
    cost)` "may pay $cost. If they don't, $action.", `repeatTimes(times, body)` "$body $times
    times", `timesPaid(which, subject)`, `choiceWasTaken(which, subject)`,
    `keywordCostPaid(label, subject)` (each "$subject's $which/$label …"), `withTotal(axis,
    comparator, bound, group)` "any number of $group with total $axis $comparator $bound",
    `beforePart(part, whose)` "before $whose $part" and `duringPart(part, whose)` "during $whose
    $part".

  The other two are rewritten by other tickets and left to them: `createTappedToken(token,
  count)` "create $count tapped $token" (`syl-token-records-to-templates`, which writes created
  tokens as nominals) and `half(rounding, amount)` "half $amount, rounded $rounding"
  (`syl-round-each-time`, which moves the rounding into Tails with the amount first). The
  `preventDamage` and `removeCounters` families are left out as well: their object becomes one
  noun phrase (`syl-prevent-damage-phrase`, `syl-counters-noun-phrase`), which replaces the
  parameters a reorder would move.
  `abilityGrantSelected` loses its subject in `syl-continuous-effect-subject`; this ticket still
  puts `selection` before `source`.
- `doIf` has no `postposed` parameter; `doOnlyIf` ("<action> if <condition>") says the printed
  order itself, and what the flag changed for reference resolution is kept by that order. That
  equivalence is not checkable until reference resolution exists; the landing lists every former
  `postposed` call with its new form.
- The reordered corpus means the same: a one-off script, kept in `docs/evidence/`, checks that
  every call in `plugins/` binds the same parameter-to-value map before and after (apart from
  `postposed`), and the landing reports the declaration and call counts it compared.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `docs/design/syl.md` §5
- `plugins/builtin/templates/ability/when.syl`, `whenever.syl`, `at.syl`,
  `plugins/builtin/templates/static/gatedBy.syl`,
  `plugins/builtin/primitives/static/staticSpecGatedBy.syl`,
  `plugins/builtin/primitives/action/doIf.syl`, `plugins/builtin/templates/action/doOnlyIf.syl`
