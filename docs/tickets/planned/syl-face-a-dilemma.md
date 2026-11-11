---
needs: []
---

# Syl: facing a dilemma

## Why

A player faces a dilemma whenever they choose one or more modes for a spell or ability, and
effects count the dilemmas a player faced this turn [CR#700.17]. Star Trek cards trigger on it
("Whenever you face a dilemma, draw a card."), check it ("if you faced a dilemma this turn") and
count it ("for each dilemma you've faced this turn"). *dilemma* is a vocabulary gap today, so
these units go unread.

## Done when

- The trigger, the this-turn condition and the count each have a declaration whose meaning is
  choosing one or more modes [CR#700.17], and each has a real caller.
- These units read in `cargo xtask english strata`: Melkotian Adjudicator, Seven of Nine, Jett
  Reno, Problem Solver, Gabriel Lorca, Mirror Menace, Obsidian Order Operative, Tell Me What You
  See, Elim Garak, Simple Tailor, The Caretaker and Captain Benjamin Sisko.

## Pointers

- `plugins/builtin/definitions/` (event and condition declarations)
- `crates/vocabulary/catalogs/`
- `docs/decisions/english.md`, `docs/decisions/semantics.md`
