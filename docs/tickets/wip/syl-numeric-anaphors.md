---
needs: [syl-single-face-card-call, cleanup-step6-spine]
---

# Decide whether `.syl`'s numeric anaphors become one primitive

## Why

`.syl` has five `Amount` primitives that read a number back from earlier text or from the
enclosing event or comparison: `thatMuch` ("that much"), `theOutcome(sort)` ("the damage
prevented this way" and similar, through Templates such as `preventedThisWay`), `groupSize` ("that
many" counting an event's or action's group, as in Woodland Champion and Divine Visitation),
`theDifference` ("the difference", as in Sandstone Oracle) and `chosenNumber` ("the chosen
number"). The first four are one kind of reference, a numeric anaphor, perhaps qualified by sort.
One primitive (or `thatMuch` with an optional sort) would give them one resolution path in the
greenfield discourse resolution; keeping them apart may be right if they resolve differently.

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

Written against the spine (`cleanup-step6-spine`).

## Design

- `thatMuch(sort: Option(OutcomeSort) = none)` merges `thatMuch`, `theOutcome` and `groupSize`, with
  `thatMany` (`spelling = "that many"`, `body = thatMuch`) for the count-noun surface; `theOutcome`
  and `groupSize` are deleted, and `preventedThisWay`, `removedThisWay`, `shortOfCeiling` and
  `infect` pass `sort = …`.
- `theDifference` stays its own primitive because a strict inequality introduces it into that
  comparison's positive branch alone, closed across negation and disjunction, and bare `thatMuch`
  never reads it.
- `chosenNumber` stays excluded because it belongs to the `ChoiceRef` family (`theChoice`,
  `theLatestChoice`) and reaches across linked abilities ([CR#607.1,607.2d]), whereas numeric
  anaphors resolve within the current ability.

## Done when

- The four readbacks (`thatMuch`, `theOutcome`, `groupSize`, `theDifference`) are either one
  primitive, optionally sort-qualified, with the old ones expressed through it or deleted, or stay
  distinct with each doc comment saying why it is not the general one. `chosenNumber` is included
  or its exclusion is stated.
- Not checkable until reference resolution exists: resolution keeps these behaviors for the
  merged or separate forms:
  - bare "that much" / "that many" is refused when two eligible numeric outcomes are in scope, and
    a sort-qualified read ("the life lost this way", "the damage prevented this way", "the
    result") picks by sort;
  - "the difference" stays scoped to its comparison branch and does not leak across negation or
    disjunction;
  - a group count resolves both a proposed event's group (Divine Visitation) and a completed
    action's group (Woodland Champion).
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/quantity/thatMuch.syl`, `theDifference.syl`, `chosenNumber.syl`;
`plugins/builtin/templates/quantity/thatMany.syl`, `preventedThisWay.syl`, `removedThisWay.syl`,
`shortOfCeiling.syl`; `plugins/canon/cards/Woodland Champion.syl`, `Divine Visitation.syl`,
`Sandstone Oracle.syl`; `docs/design/semantics.md` §1 (item 5(b)), §2.4.
