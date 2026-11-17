---
needs: []
---

# Restrict `PastParticipialAbsolute` to overt Subjects and `Predicate` hosts

## Why

`PastParticipialAbsolute` (`AbsoluteClause -> subject:ComplementNounPhrase VerbPhrase`) and the
`AbsoluteClause` frames on `with` and `without` (`hosts: NominalAndPredicate`) add 2,191 derivations
across 83 Corpus Units, with 0 Corpus Units needing them for coverage. Only 22 units are genuine
augmentative *with* absolute clauses (*"Play with your hand revealed"*, *"Play with the top card of
your library revealed"*; CGEL 14§8.3:1262), all hosted on a `Predicate` and all with an overtly
determined Subject. The remaining ~2,090 derivations are false positives:

- Bare undetermined Subjects (`Determination = Bare`, ~1,300+ derivations): in *"cast a spell with
  mana value less than or equal to the number of Elves and Faeries you control from among cards
  exiled with Maralen this turn"* (*Maralen, Fae Ascendant*, 594 derivations; *Sunbird's Invocation*,
  150; *Bring to Light*, 120; *Tetsuo, Imperial Champion*, 120; *Mockingbird*, 118; *Cemetery
  Prowler*, 100), `with` takes bare `"mana value … from among cards"` or `"cards"` as the Subject of
  an absolute clause whose predicate is `"exiled with Maralen"`.
- `Nominal`-hosted `AbsoluteClause` (~700+ derivations): where `with` modifies a `Nominal`
  (*Protective Sphere*'s `"the mana spent"`, 127; *Starfleet Communicator*, 112; *Cemetery
  Illuminator*'s `"a card type with [a card] [exiled with this creature]"`, 55; *Aether Searcher*,
  50; *Holistic Wisdom*, 32), `with + NP` (`ParticipialPostmodifier` / `SingleParticipialPostmodifier`)
  already reads the participial dependent on the noun, and `with + AbsoluteClause` duplicates it or
  attaches across a selected `with` complement (`share … with`).

## Design

- Require `subject.Determination := Overt` on `PastParticipialAbsolute` in `english.fug` (admission
  gate, `english.md` §6, naming *Maralen, Fae Ascendant* and *Cemetery Prowler*).
- Change `(complement: AbsoluteClause, hosts: NominalAndPredicate)` on `with` and `without` in
  `prepositions.ron` to `hosts: Predicate`.

## Done when

- All *"play(s) with your/their hand(s) / the top card … revealed"* cards and
  `crates/english/judgments/play_with_absolute.txt` continue to read with their `AbsoluteClause`
  adjunct on `play`.
- *Maralen, Fae Ascendant*, *Sunbird's Invocation*, *Protective Sphere*, *Bring to Light*, *Tetsuo,
  Imperial Champion*, *Mockingbird*, *Starfleet Communicator* and *Cemetery Prowler* lose their
  spurious `PastParticipialAbsolute` Readings, with 0 verdict regressions in the census.
- `cargo xtask english strata --check`, `cargo test -p sylex_vocabulary -p sylex_english` and
  `cargo clippy -p sylex_vocabulary -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: `PastParticipialAbsolute`, `slot PP`.
- `crates/vocabulary/lexicon/english/prepositions.ron`: `with`, `without`.
- `crates/english/judgments/play_with_absolute.txt`.
- `docs/design/english.md` §6.
