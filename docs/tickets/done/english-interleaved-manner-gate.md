---
needs: []
---
# A Category and a valence hook that let *this way* precede only a *by* phrase

## Why

`InterleavedManner` puts a manner Noun Phrase (*this way*) before a passive's Internalised
Complement only, through `category InternalisedInterleaved` and the `interleaved_internalised`
hook in `valence VerbPhrase`; the general `Interleaved` Category takes Preposition Phrases and
temporal Noun Phrases before any later Complement. The comment's only ground is the style guide
(*this way* right after its verb), which ADR §6 says is never a gate's justification. It serves
one card (*weren't chosen this way by any player*, Raiding Party). The *by* phrase is a non-core
Complement (CGEL Ch. 4 §3.2.1, p. 241), but CGEL's core/non-core line (Ch. 4 §1, p. 216) puts
every goal or source Preposition Phrase on the non-core side too, so the distinction drawn
(*by* only) is narrower than any CGEL class. The gate also appears to refuse grammatical text
such as *deals 3 damage this way to any target*.

## Design

- The gate stays: without it, *this way* also attaches to the higher verb before that verb's
  Preposition Phrase Complement wherever an Object ends in a participle or relative clause.

## Done when

- The gate is ablated (manner Noun Phrase allowed before any later Complement) and the
  spurious Readings and unit growth on the supported corpus are recorded.
- The construct follows the owner's ruling; if kept, its comment names Raiding Party and the
  structure the gate prevents on a named card, with no style-guide ground.
- Raiding Party keeps a correct Reading, and the census shows no Reading lost on the supported
  corpus.

## Pointers

- `crates/english/grammar/english.fug`: `InterleavedManner`, `category InternalisedInterleaved`,
  `category Interleaved`, `valence VerbPhrase` (`interleaved_internalised`).
- `crates/syntax/src/compile.rs` (the `interleaved_internalised` derivation).
- `docs/decisions/english.md` §4 to §6.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F9),
  `docs/evidence/english-fug-hax-check/gemini.md` (D1).
