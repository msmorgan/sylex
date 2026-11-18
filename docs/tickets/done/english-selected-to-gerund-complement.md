---
needs: []
---

# A verb's selected *to* phrase reads "to attacking creatures" as *to* plus a gerund-participial

## Why

Where a head's frame selects a *to* phrase and a participial premodifier follows *to*, the forest
also reads the participle as a gerund-participial Complement of *to*: "attached to [attacking
creatures your opponents control]" (*Remove Enchantments*, under both verb and adjective
*attached*) and "damage that would be dealt to [attacking creatures you control]" (*Dolmen Gate*,
*Iroas, God of Victory*, *Losheel, Clockwork Scholar*, *Goldbug, Humanity's Ally*) each have
*attacking* as a verb with "creatures" its Object, beside the Noun Phrase the card means. On
*Dolmen Gate* line 1 that reading is in 2 of 4 Derivations. On *Remove Enchantments* line 1, of
4,096 Derivations, the first *attacking* is the Complement of *to* in 2,240 and the second in
2,304, some of them with *attacking* intransitive and "creatures your opponents control" an
Object postposed past "to attacking".

A gerund-participial Complement of *to* is what the pool prints after "in addition" (*in addition
to paying its other costs*), where no verb selects the *to*. Those five cards are the only ones
with it under a head's selected *to*, and it is the only analysis in none of them.

*Of* has the same overgeneration: its Gerund frame is taken under every head that selects *of*,
so "the number of [attacking creatures]" (25 units: *Akroan Hoplite*, *Dogpile*, *Camouflage*,
...) and "the total power of [attacking creatures]" (*Klauth, Unrivaled Ancient*) read
*attacking* as a verb too. The real gerund-participial Complements of a selected *of* are under
*instead* (*instead of putting it into its owner's graveyard*, 54 units) and *part* (*as part of
casting a spell they control*, *Standard Bearer*, *Coalition Flag*, *Coalition Honor Guard*).

Reproduce with `cargo xtask english forestwalk --card 'Dolmen Gate' --line 1` and read the options
of `dependency "attacking"#1`.

## Design

- The gate is on what a selected Preposition Phrase may take as its Complement, not on a lexeme,
  and it is the grammar's existing Complement Constraint `KindSelected` (`english.md` §3): the
  Gerund frames of *to* and *of* name it, so only a Frame Slot that names the Gerund kind,
  `PPWith([...], Gerund)`, selects them. A slot that names *to* alone keeps every other kind
  *to* takes (an Amount, a Noun Phrase, the bare Nominal of *return ... to hand*), so the gate
  is not "a verb's *to* slot takes a Noun Phrase".
- A preposition a frame selects with a gerund-participial Complement on real card text keeps it;
  the census names each one before the gate is drawn: *in addition* (*to*), *instead* and *part*
  (*of*), which name `PPWith(["to"], Gerund)` and `PPWith(["of"], Gerund)`. The gate's ablation
  is reported (`english.md` §6).

## Done when

- *Dolmen Gate* line 1 and *Remove Enchantments* line 1 have no Derivation in which *attacking* is
  the Complement of *to*, and *Akroan Hoplite* line 1 none in which it is the Complement of *of*.
- "In addition to paying its other costs" keeps its gerund-participial Complement (*Demonic
  Embrace*), as do "instead of putting it ..." (*Syncopate*) and "as part of casting a spell"
  (*Standard Bearer*).
- A Judgment entry on real card text witnesses each of these.
- The census before and after shows no face lost and no unit with more Readings; every unit whose
  verdict changes is checked, and the units the gate changes are counted.
- `cargo xtask english strata --check`, `cargo test -p sylex_english`, `cargo test -p
  sylex_syntax` and clippy on both pass.

## Pointers

- `crates/english/grammar/english.fug`: `constraint KindSelected`, and the Gerund Complement
  variants of `valence PrepositionPhrase` (`PrepositionPhrase/Gerund@ComplementOnly`).
- `crates/vocabulary/lexicon/english/prepositions.ron`: the frames of *to*, *of*, *in addition*
  and *instead*; `nouns.ron`: *part*.
- `crates/syntax/src/compile.rs`: how a frame's `PP` slot fill becomes a `Selected` child.
- `crates/english/README.md`, "Working rules", "The forest database"; `docs/design/english.md`
  §3, §6.
