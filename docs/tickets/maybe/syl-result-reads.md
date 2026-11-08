---
needs: []
---

# Consolidate `.syl`'s vote, roll, coin-flip and secret-choice result reads

## Why

Single-mechanic result reads are scattered across `Predicate`, `Condition`, `Amount` and
`Action` primitives, though `semantics.md` §1 item 5(b) prefers composing existing primitives:
- votes [CR#701.38a]: `withMostVotes` (Council's Judgment, Custodi Squire), `voteLead(label,
  orTied)` (Coercive Portal, Plea for Power, Tyrant's Choice), `votesFor(label)` (Lieutenants of
  the Guard, Orchard Elemental);
- secret numbers: `choseExtreme(op)` (Menacing Ogre);
- coins [CR#705.2]: `coinCameUp(face)`, `coinsShowing(face)`;
- dice [CR#706.2,706.3a,706.5]: `anyResultIs(comparator, bound)`, `rolledDoubles`,
  `applyResultsTable(rows)` (Thunderwave);
- stored results [CR#706.8a..706.8c]: `storeResults`, `rerollStored`, `greatestStoredMatch`
  (Centaur of Attention).

`coinCameUp`, `coinsShowing`, `anyResultIs` and `rolledDoubles` have no user in canon.

## Done when

- `withMostVotes` and `voteLead` are expressed through `votesFor` with `superlative` /
  `compareOver`, keeping the strict "more votes" and the "more votes or the vote is tied" readings
  apart, or each stays with its doc comment saying why it cannot be.
- `choseExtreme` is expressed through `superlative` / `compareOver` over `chosenNumber`, or stays
  with the reason stated.
- The coin and die reads share one typed result read, or each kept one states why.
  `ignoreOutcomes` [CR#706.6] and `shiftResult` [CR#706.2] stay operations on rolls.
- The canon cards named above still read as printed.

## Not in scope

Deleting unused primitives for being unused; that is the `.syl` dead-weight
sweep.

## Pointers

`plugins/builtin/primitives/predicate/` (`withMostVotes`, `choseExtreme`,
`coinCameUp`, `superlative`, `compareOver`), `plugins/builtin/primitives/condition/` (`voteLead`,
`anyResultIs`, `rolledDoubles`), `plugins/builtin/primitives/quantity/` (`votesFor`,
`coinsShowing`, `greatestStoredMatch`, `chosenNumber`), `plugins/builtin/primitives/action/`
(`storeResults`, `rerollStored`, `applyResultsTable`, `ignoreOutcomes`),
`plugins/builtin/templates/action/shiftResult.syl`; the canon cards above.
