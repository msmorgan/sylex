---
needs: []
---

# Is "of creatures" a complement of "abilities"?

## Why

`crates/vocabulary/lexicon/english/nouns.ron` declares `ability` with the frame
`Complement(PP(["of"]))`, so *Cursed Totem*'s "Activated abilities of creatures" parses "of
creatures" as a complement the noun selects (`Valence:BareNominal/NounComplement(PP[prep:of])`),
the frame `half`, `choice`, `kind` and `controller` have. The phrase names the abilities' source
[CR#113.7] and alternates with a genitive ("a creature's abilities"), which may make it a
modifier any noun could take instead. The frame decides which Readings exist wherever "abilities
of …" meets another post-head dependent, and what attaches to what in them.

## Open questions

- **Complement or modifier.** Answer from CGEL's tests for a noun's complements (the
  `cgel-reference` skill, with page citations) applied to the pool's "abilit(y|ies) of …" units,
  and say what each answer does to their Readings.

## Done when

- The open question is answered with its CGEL citation and the units that show it, and
  `nouns.ron` and the grammar read "abilities of …" accordingly.
- The same test is applied to the other nouns in `nouns.ron` whose only frame is
  `Complement(PP(["of"]))`, and each that fails it is listed and changed, or ticketed.
- The census shows which Readings changed on the supported corpus and why, and `cargo xtask
  english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for every
  changed crate.

## Pointers

`crates/vocabulary/lexicon/english/nouns.ron`; `crates/english/grammar/english.fug` (the noun
complement productions); `cargo xtask english forestwalk --card "Cursed Totem" --line 1`;
`docs/design/english.md`.
