---
needs: []
---

# Read a relative or participial clause over a whole coordination of Noun Phrases

## Why

A relative or participial clause printed after coordinated Noun Phrases that each bring their own
determiner attaches only inside the last conjunct. *Harmonic Prodigy*'s "a triggered ability of a
Shaman or another Wizard you control" means "you control" of both the Shaman and the Wizard
(`pragmatics.md` §4 lists it wide), and its forest has "you control" only as a Modifier of
"Wizard" or of "ability". "Whenever a Shaman or another Wizard you control dies, draw a card."
has one Reading, the narrow one, while "Whenever a Shaman or Wizard you control dies, draw a
card." has both, through the Nominal coordination. A dependent after a coordination may apply to
the adjacent coordinate or to the whole coordination (CGEL, Ch. 15 §1.3.4, pp. 1285–1286), and a
Preposition Phrase already does both here (`CoordinatedNounPhrasePostmodifier`); the layers
above cannot choose a Reading the forest lacks (`english.md` §4, §8).

Reproduce with
`cargo xtask english forestwalk --card 'Harmonic Prodigy' --line 2` and read the options of
`dependency "control"#1`.

## Design

- A `Postmodifier` (a relative clause or a participial clause) modifies a coordination of Noun
  Phrases on the terms `CoordinatedNounPhrasePostmodifier` sets for a Preposition Phrase: only a
  coordination, never an all-bare one, whose wide site is the Nominal coordination's, so one
  analysis keeps one tree.
- A subject relative agrees with the coordination's Number (*a creature and a land that are
  tapped*, *a creature or a land that is tapped*), and the modifier's NounKind is the
  coordination's, as `NominalPostmodifier` requires of a Nominal.
- Whether a coordination with a personal pronoun or a proper name among its conjuncts
  (`Postmodification = Closed`) takes the wide clause is settled by the gate's ablation
  (`english.md` §6): the gate stays if without it real cards gain spurious Readings such as
  *[you or a planeswalker] you control*.

## Done when

- *Harmonic Prodigy* line 2 has a Derivation in which "you control" modifies the coordination "a
  Shaman or another Wizard", and `forestwalk` narrows to it with `--pick`.
- Judgment entries witness the wide and the narrow Reading on real card text for a bare relative,
  a marked subject relative with its agreement, and a participial clause, where the cards print
  each over coordinated determined Noun Phrases.
- No all-bare coordination gains a second tree of its wide Reading: *Creatures and Vehicles you
  control gain trample until end of turn.* keeps its 2 Derivations.
- The census before and after shows no face lost, and every unit whose verdict changes is
  checked; the ablation of each gate the Production carries is reported.
- `pragmatics.md` §4 no longer says a hypothesis reads *Harmonic Prodigy* from a forest that
  lacks its wide Reading, if any sentence there depends on that.
- `cargo xtask english strata --check`, `cargo test -p sylex_english`, `cargo test -p
  sylex_syntax` and clippy on both pass.

## Not in scope

- Choosing between the wide and the narrow Reading: that is pragmatics (`pragmatics.md` §4).
- A premodifier over coordinated heads (`english-premodifier-over-coordinated-heads`).

## Pointers

- `crates/english/grammar/english.fug`: `CoordinatedNounPhrasePostmodifier`,
  `NominalPostmodifier`, the `Postmodification` feature and the `coordination NounPhrase` blocks.
- `crates/english/README.md`, "Working rules"; `crates/english/judgments/README.md`.
- `docs/design/english.md` §4, §6, §8; `docs/design/pragmatics.md` §4.
