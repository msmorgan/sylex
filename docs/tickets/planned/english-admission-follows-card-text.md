---
needs: []
---

# Admission follows card text, not CGEL's inventory of grammatical forms

## Why

`docs/design/english.md` §4 says "A grammatical sentence a card could print is admitted", and
work has read that as a duty to admit every form CGEL calls grammatical: a value kept so that
*another day* reads, a gate deleted because CGEL allows the form it refused, a Production
proposed for *another three bodies*. No card prints those. CGEL shaped the first implementation;
taken as a list of forms the grammar must admit, it now produces spurious Readings.

## Design

- §4 says what the grammar is for: reading what Magic cards print. A form is admitted because
  card text has it. CGEL is the source for analyzing a form cards print (what its parts are and
  how they attach), and for telling a grammatical Reading from an ungrammatical one; it is not a
  list of forms to admit.
- A form no card prints needs no Production, and a gate is not wrong because it refuses one. A
  gate still needs its measured effect on real card text (§6).
- Custom cards are read by the same grammar: a custom card's wording that no printed card shares
  is a reason to extend the grammar when that card exists, not before.

## Done when

- `docs/design/english.md` §4 (and §6, §8 where they repeat the charter) state the above as
  current fact, and `crates/english/README.md` "Working rules" agrees with them.
- Comments in `crates/english/grammar/english.fug` that justify a Production or a retained value
  only by a CGEL example no card prints are listed; each is reworded, or its Production or value
  is ticketed for removal.

## Pointers

- `docs/design/english.md` §4, §6, §8; `crates/english/README.md`.
- `crates/english/grammar/english.fug`: `feature DeterminativeDistribution`
  (`AlternativeAdditive`), `FloatedAll`.
