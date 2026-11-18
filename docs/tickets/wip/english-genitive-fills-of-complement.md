---
needs: []
---

# A genitive and an *of* Complement fill one argument twice

## Why

CGEL counts a genitive subject-determiner among an NP's complements: *the flood's consequences*
and *the consequences of the flood* each have one complement, the cause, in two forms (Ch. 5,
§16.5.3, p. 475, [56ii]), and in such a monovalent NP the two compete (p. 476, [59]). A noun whose
*of* frame names the one argument a genitive can name (*hand*, *library*, *graveyard*, *life
total*, *mana value*, *name*, *ability*, *power*, *top*) can therefore take one or the other, never
both, yet the grammar lets a genitive-determined head take its *of* Complement too. Right Node
Raising then shares an *of* phrase onto it: in One with the Multiverse line 3, *your hand or the
top of your library* reads *[your hand or the top] of your library* (5 of 13 Derivations); in
Serene Master line 1 *its power and the power of target creature* shares *of target creature*
onto *its power* (2 of 8); and in 32 units that print *their choice of the top or bottom of their
library* (Misleading Motes, Aether Gust, Tolarian Contempt) *their* determines *[choice of the top]
or [bottom]* with *of their library* shared onto *bottom*, half of each unit's Derivations. Each
is a second tree no English reader builds. (Wheel of Potential line 2, *exile their hand and draw a
number of cards*, no longer shares *of cards* onto *their hand* at this ticket's base: 168
Derivations, none with that tree.)

*Mana cost*'s *for* or *of* frame names the object whose cost it is (*the mana cost for spells you
cast*), but it selected any *for* phrase, so *without paying its mana cost for as long as it
remains exiled* (Spelljack and six more) read the duration as that Complement only; the rule
above would leave those units no Reading. The frame selects a Noun Phrase, and the duration
attaches to the Verb Phrases it can modify (Spelljack 4 to 10 Derivations).

## Done when

- A head whose genitive determiner names the argument its *of* frame names takes no *of*
  Complement, directly or shared, by a rule whose comment cites CGEL; a noun with two arguments
  (*Van Dyck's portrait of Charles I*, p. 475, [56i]) is not affected.
- One with the Multiverse line 3, Serene Master line 1 and the *their choice of the top or bottom
  of their library* units lose those Readings; the census shows which Readings changed and why,
  and `cargo xtask english strata --check`, `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for every changed crate.

## Pointers

`crates/vocabulary/lexicon/english/nouns.ron`; `crates/english/grammar/english.fug`
(`valence BareNominal`, the genitive determiner productions); `crates/syntax/src/compile.rs`
(the `gap0` noun Complement variants);
`cargo xtask english forestwalk --card "One with the Multiverse" --line 3`.
