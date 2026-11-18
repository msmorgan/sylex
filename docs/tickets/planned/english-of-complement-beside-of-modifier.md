---
needs: []
---

# *Color*, *type*, *source* and *turn* take both a Complement and a modifier *of* phrase

## Why

*The colors of mana spent to cast this spell* (34 units) and *the land types of the sacrificed
land* name a property's bearer, a Complement (CGEL, Ch. 5, §14.1, p. 442, [7i]), as *colorless
sources of damage* (Ghostly Flame) names what the source is the source of; *a color of your
choice* (58 units), *a creature type of your choice* (22) and *a source of your choice* (65)
describe the head, a modifier (§14.2, p. 446, [14i]). *Color*, *type* and *source* declare no
frame, so the grammar reads every one of them as a modifier. Declaring `Complement(PP(["of"]))`
reads every one as a Complement instead: same-head complement priority (`docs/design/english.md`
§3) makes a Preposition Phrase that one of a head's frames selects never also its modifier, so
*of your choice* loses its modifier reading.

*Turn* meets the same rule from the other side. *Your first, second, or third turns of the game*
(4 units) names the whole a turn is part of, a Complement ([7vi]); but a bare head takes a
Preposition Phrase right after it that its frame selects as that Complement, never as another
host's dependent, so with the frame *of the chosen type* in *The next spell you cast this turn of
the chosen type* (Apex Observatory line 2) can no longer modify *spell*, and the unit has no
Reading.

A frame cannot tell these apart: each oblique is a determined Noun Phrase. What does is a semantic
selection restriction on the Complement (§14.1(g), p. 443): a choice bears no color, and a type is
no game a turn is part of.

## Done when

- *of mana*, *of the sacrificed land* and *of damage* are Complements of *color*, *type* and
  *source*, *of the game* is *turn*'s, and *of your choice* and *of the chosen type* stay
  modifiers, on every unit that prints them, by a rule whose comment cites CGEL; or the
  ticket states, with evidence, why the grammar cannot draw the line.
- The census shows which Readings changed and why, and `cargo xtask english strata --check`,
  `cargo test` and `cargo clippy --tests -- -D warnings` pass for every changed crate.

## Pointers

`crates/vocabulary/lexicon/english/nouns.ron` (*color*, *type*, *source*, *turn*);
`docs/design/english.md` §3, "Same-head complement priority"; `crates/syntax/src/compile.rs` (`omits_complement`);
`cargo xtask english forestwalk --card "Together as One" --line 1`. An *of* frame competes with a
genitive determiner unless its noun has a `distinct_genitive` (`docs/design/english.md` §3, "A
genitive and an *of* Complement compete"): *your first, second, or third turns of the game* needs
*turn* to have one, as the phases and steps do.
