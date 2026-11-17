---
needs: []
---

# Numeric anaphors: the two-antecedent rule and "this way" across an additional cost

## Why

`semantics.md` §2.4 and `thatMuch`'s doc comment say an unqualified `thatMuch` ranks the numeric
antecedents in scope by clause prominence and is refused only when two tie, a rule nobody chose.
*Neheb, Dreadhorde Champion* ("you may discard any number of cards. If you do, draw that many
cards and add that much {R}") has the combat damage, the discard count and the draw count in
scope.

*Burn at the Stake* and *Explosive Singularity* count the creatures tapped for their additional
cost with `was(tap(which), within = thisWay)`. The tap stands in a separate
`static(addedCost(…))`, and §2.4 says "this way" reads only the current ability's stamps. Both
cards print "tapped this way". *Explosive Singularity*'s reduction is also applied when the total
cost is determined [CR#601.2f], before the creatures are tapped as the cost is paid [CR#601.2h].

`shortOfCeiling` has no caller, so nothing exercises the `ceilingShortfall` sort that §2.4 names
as a sort-qualified read. *Archon of Absolution*'s "pays {1} for each of those creatures" is
written `times(1, numberOf(those(creature)))`, which spells "the number of those creatures" and,
inside an Unless Condition read per participant, may count one. `theDifference`'s doc comment and
§2.4 say a difference is introduced into "that comparison's branch", which `compareOver`, a
`Nominal` filter, does not have, though *Boreas Charger* and *Tales of the Ancestors* read the
difference a `compareOver` description admits its members by.

## Done when

- `semantics.md` §2.4 and `thatMuch`'s doc comment claim no ranking among numeric antecedents, and
  §6 states that the choice is open with the readings any rule must give. *Neheb, Dreadhorde
  Champion* is in canon with the bare anaphors it prints.
- §2.4 lets "this way" read a cost paid to cast the same spell or a cost the same ability
  imposes, and §3.1 says a cost reduction reads the announced payment. *Burn at the Stake* and
  *Explosive Singularity* stay as written.
- `shortOfCeiling` has a real Vintage-legal caller, or it and `ceilingShortfall` are deleted and
  §2.4 no longer names them.
- *Archon of Absolution* prints "for each of those creatures" and counts every one of them.
- `theDifference`'s doc comment and §2.4 name the constructs that introduce a difference, say how
  a `compareAmt` difference reaches the consequent given §2.2's "closed on exit", and say that a
  `compareOver` difference is read with its member wherever the member is read.
- `thatMuch`'s doc comment cites rules that support a completed event's group count and a `would`
  clause's prospective amount, where it now cites [CR#608.2c] and [CR#614.6].
- `semantics.md` §3.3 cites infect's marking rule [CR#702.90c] for the marking result.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `plugins/builtin/primitives/quantity/thatMuch.syl`, `theDifference.syl`;
  `plugins/builtin/templates/quantity/thatMany.syl`, `shortOfCeiling.syl`
- `plugins/canon/cards/Burn at the Stake.syl`, `Explosive Singularity.syl`,
  `Archon of Absolution.syl`, `Sandstone Oracle.syl`
- `docs/design/semantics.md` §2.2, §2.4, §3.1, §3.3, §6
