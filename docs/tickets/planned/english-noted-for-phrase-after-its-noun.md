---
needs: []
---

# *The last noted life total for this enchantment* has no Reading with *for* inside it

## Why

*Sigarda's Splendor* line 2 prints "draw a card if your life total is greater than or equal to
the last noted life total for this enchantment". The life total is the one noted for this
enchantment, so *for this enchantment* belongs inside the Noun Phrase. The unit has two
Derivations, and in both the phrase is an Adjunct of a verb, *draw* or *is*; neither has the
intended analysis, so the unit has no correct Reading (`docs/design/english.md` §8).

Two things keep the phrase out. *Life total* declares `Complement(PPWith(["of"], NP))` alone,
where *mana cost* declares `PPWith(["for", "of"], NP)`; and an unselected *for* phrase is hosted
by a Predicate only, never a Nominal.

Ten other units print *note* with a *for* phrase after the participle: *the greatest number noted
for it this turn* (Bell Borca, Spectral Sergeant), *the highest number you noted for cards named
Lurking Automaton*, *a creature type that hasn't been noted for this Saga* (Long List of the
Ents), *each creature type noted for target permanent you control* (Volo, Itinerant Scholar). In
each, the *for* phrase is a dependent of *noted*. On Sigarda's Splendor *noted* stands before the
noun as a premodifier, and its *for* phrase follows the noun.

## Done when

- Sigarda's Splendor line 2 has a Reading in which *for this enchantment* is inside *the last
  noted life total for this enchantment*, and the landing says which word it depends on there
  (*life total* or *noted*) and why, from how the ten units above read and from what else the
  choice admits on real cards.
- The ten *noted for* units keep the Readings they have.
- The census shows which Readings changed and why, with no verdict loss, and
  `cargo xtask english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings`
  pass for every changed crate.

## Pointers

- `crates/vocabulary/lexicon/english/nouns.ron` (`life total`, `mana cost`),
  `crates/vocabulary/lexicon/english/prepositions.ron` (`for`),
  `crates/vocabulary/lexicon/english/verbs.ron` (`note`).
- `cargo xtask english forestwalk --card "Sigarda's Splendor" --line 2`.
- `docs/tickets/planned/english-set-comparison-of-phrase.md`: another phrase after a noun that a
  word before the noun licenses.
