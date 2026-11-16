---
needs: []
---

# Numeric anaphors: the two-antecedent rule and "this way" across an additional cost

## Why

`semantics.md` §2.4 and `thatMuch`'s doc comment say an unqualified `thatMuch` ranks the numeric
antecedents in scope by clause prominence and is refused only when two tie. Nothing else has
settled that rule: the alternative is to refuse whenever two eligible outcomes are in scope.
*Neheb, Dreadhorde Champion* ("you may discard any number of cards. If you do, draw that many
cards and add that much {R}") has the combat damage and the discard count both in scope.

*Burn at the Stake* and *Explosive Singularity* count the creatures tapped for their additional
cost with `was(tap(which), within = thisWay)`. The tap stands in a separate
`static(addedCost(…))`, and §2.4 says "this way" reads only the current ability's stamps. Both
cards print "tapped this way". *Explosive Singularity*'s reduction is also applied when the total
cost is determined [CR#601.2f], before the creatures are tapped as the cost is paid [CR#601.2h].

`shortOfCeiling` has no caller, so nothing exercises the `ceilingShortfall` sort that §2.4 names
as a sort-qualified read. *Archon of Absolution*'s "pays {1} for each of those creatures" is
written `times(1, numberOf(those(creature)))`, which spells "the number of those creatures" and,
inside an Unless Condition read per participant, may count one; *Ghostly Prison* writes the same
phrase with `forEach`. `theDifference`'s doc comment and §2.4 name `compareOver` as introducing a
difference, but `compareOver` is a `Nominal` filter with no branch, and both callers use
`compareAmt`.

## Open questions

- With two eligible numeric antecedents in scope, is bare "that much" resolved by clause
  prominence and refused on a tie, or always refused?
- How does "this way" reach the objects a spell's additional cost tapped: an exception in §2.4
  for the spell's own costs, or another form for these two cards? And what does *Explosive
  Singularity*'s reduction read when the total cost is determined?

## Done when

- The open questions are answered in `semantics.md` §2.4 and the declarations' doc comments.
  *Burn at the Stake* and *Explosive Singularity* are written to the answers, and *Neheb,
  Dreadhorde Champion*, which canon lacks, is authored to them.
- `shortOfCeiling` has a real Vintage-legal caller, or it and `ceilingShortfall` are deleted and
  §2.4 no longer names them.
- *Archon of Absolution* prints "for each of those creatures" and counts every one of them.
- `theDifference`'s doc comment and §2.4 name only the constructs that introduce a difference,
  and §2.4 says how the difference reaches the consequent given §2.2's "closed on exit".
- `thatMuch`'s doc comment cites rules that support a completed event's group count and a `would`
  clause's prospective amount, where it now cites [CR#608.2c] and [CR#614.6].
- `semantics.md` §3.3 cites infect's marking rule [CR#702.90c] for the marking result.
- `docs/tickets/wip/syl-counters-noun-phrase.md`, or wherever that ticket then is, quotes *Hardened
  Scales* and *Winding Constrictor* as `plus(thatMany, 1)`.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `plugins/builtin/primitives/quantity/thatMuch.syl`, `theDifference.syl`;
  `plugins/builtin/templates/quantity/thatMany.syl`, `shortOfCeiling.syl`
- `plugins/canon/cards/Burn at the Stake.syl`, `Explosive Singularity.syl`,
  `Archon of Absolution.syl`, `Ghostly Prison.syl`, `Sandstone Oracle.syl`
- `docs/design/semantics.md` §2.2, §2.4, §3.3
