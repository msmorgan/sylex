---
needs: []
---

# A verb's selected *to* phrase reads "to attacking creatures" as *to* plus a gerund-participial

## Why

Where a verb's frame selects a *to* phrase and a participial premodifier follows *to*, the forest
also reads the participle as a gerund-participial Complement of *to*: "attached to [attacking
creatures your opponents control]" (*Remove Enchantments*) and "damage that would be dealt to
[attacking creatures you control]" (*Dolmen Gate*, *Iroas, God of Victory*, *Losheel, Clockwork
Scholar*, *Goldbug, Humanity's Ally*) each have *attacking* as a verb with "creatures" its Object,
beside the Noun Phrase the card means. On *Remove Enchantments* line 1 that reading and its
variants are in 3,284 of 4,096 Derivations, 624 of them with *attacking* intransitive and
"creatures your opponents control" an Object postposed past "to attacking".

A gerund-participial Complement of *to* is what the pool prints after "in addition" (*in addition
to paying its other costs*), where no verb selects the *to*. Those five cards are the only ones
with it under a verb's selected *to*, and it is the only analysis in none of them.

Reproduce with `cargo xtask english forestwalk --card 'Dolmen Gate' --line 1` and read the options
of `dependency "attacking"#1`.

## Design

- The gate is on what a frame's selected Preposition Phrase may take as its Complement, not on a
  lexeme: a slot a verb frame fills with a *to* phrase takes a Noun Phrase.
- A preposition a frame selects with a gerund-participial Complement on real card text keeps it;
  the census names each one before the gate is drawn, and the gate's ablation is reported
  (`english.md` §6).

## Done when

- *Dolmen Gate* line 1 and *Remove Enchantments* line 1 have no Derivation in which *attacking* is
  the Complement of *to*.
- "In addition to paying its other costs" keeps its gerund-participial Complement (*Demonic
  Embrace*).
- A Judgment entry on real card text witnesses each of the two.
- The census before and after shows no face lost and no unit with more Readings; every unit whose
  verdict changes is checked, and the units the gate changes are counted.
- `cargo xtask english strata --check`, `cargo test -p sylex_english`, `cargo test -p
  sylex_syntax` and clippy on both pass.

## Pointers

- `crates/english/grammar/english.fug`: the `valence PrepositionPhrase` block and its `Gerund`
  Complement variants (`PrepositionPhrase/Gerund@ComplementOnly`).
- `crates/syntax/src/compile.rs`: how a frame's `PP` slot fill becomes a `Selected` child.
- `crates/vocabulary/lexicon/english/verbs.ron`: `TransitiveWithPP(["to"])`.
- `crates/english/README.md`, "Working rules", "The forest database"; `docs/design/english.md`
  §3, §6.
