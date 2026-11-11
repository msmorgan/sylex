---
needs: []
---
# *Outside the game* is a noun, with a fixed-frame apparatus around it

## Why

The lexicon enters *outside the game* as a singular count noun with `distribution: FixedFrame`,
and *from* selects it through a `FixedFrameNP` frame. Five grammar declarations exist for that
one lexeme: the `FixedFrame` value of `NounDistribution`, the `FixedFrameNP` value of
`ComplementKind`, `category FixedFrameNounPhrase`, `slot FixedFrameNP` and
`FixedFrameBareNoun`. The lexicon comment cites CGEL Ch. 5 §8.5(b), pp. 409–410 for bare nouns
in fixed frames; that page is about a bare noun after a preposition (*in hospital*, *by bus*),
and *outside the game* is the preposition *outside* with its Noun Phrase Complement. CGEL
analyzes *from* plus such a phrase as a preposition taking a Preposition Phrase Complement
(*from behind the curtain*, Ch. 7, pp. 599, 617, 624). The grammar already has `word Outside`
and the lexicon a preposition *outside* with an NP frame. The lexicon comment gives the noun's
other motive: *search your library and/or outside the game* (Invasion of Arcavios, Turtles
Forever), read as Nominals coordinated under *your*.

## Open questions

- For the two *search ... library and/or outside the game* cards: a coordination of unlike
  categories (Noun Phrase and Preposition Phrase) under *search*, or keep a noun for those two
  cards with a comment that names them?

## Done when

- CGEL pp. 409–410 and Ch. 7 pp. 599, 617, 624 are re-read and the claims above confirmed or
  refuted; if refuted, the ticket closes with no change.
- *From outside the game* is *from* with a Preposition Phrase Complement headed by *outside*
  (`PP(["outside"])` in *from*'s frames), and every *from outside the game* unit keeps a correct
  Reading.
- The two *search* cards are handled as the owner rules, and the declarations above, the
  `FixedFrame` lexicon distribution and *from*'s `FixedFrameNP` frame are deleted if nothing
  else uses them.
- The census shows no Reading lost on the supported corpus.

## Pointers

- `crates/english/grammar/english.fug`: `feature ComplementKind`, `feature NounDistribution`,
  `category FixedFrameNounPhrase`, `slot FixedFrameNP`, `FixedFrameBareNoun`, `word Outside`.
- `crates/vocabulary/lexicon/english/nouns.ron` (*outside the game*),
  `crates/vocabulary/lexicon/english/prepositions.ron` (*from*, *outside*).
- [CR#400.11].
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F1),
  `docs/evidence/english-fug-hax-check/gemini.md` (A1).
