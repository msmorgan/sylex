---
needs: []
---

# English: a central adverb between *be* or a copula and its Complement

## Why

An adverb in central position follows an auxiliary and precedes its Complement. The grammar
admitted one after *be* only before a predicative Complement, through a hand-written Production,
and never before *be*'s locative Complement. So Commander Spock's "If that card is still on top
of your library, draw a card." had no Reading, and *is still on the battlefield* read as a
predicative while *is on the battlefield* read as a locative Complement.

## Design

- The verb Valence schema derives a phrase with a central Adjunct between the verb and its first
  Complement from each non-catenative frame of a central head and from each frame of a central
  frame family. The grammar declares the Adjunct's function Category (`central`), the central
  heads (`central_heads: BeVerb`, *be* alone) and the central families (`central_families:
  [Copular]`).
- A lexical copula's central adverb comes from its Copular frame (*have it become no longer
  suspected*, Deadly Complication); no hand-written post-copular Production remains.
- *is still on the battlefield* and *is still on top of your library* read as *be* with its
  locative Complement and a central Adjunct, as the adverb-less forms do.

## Done when

- The sentence reads with *still* scoping over the locative predicate, and Commander Spock is read
  in `cargo xtask english strata`.
- Judgments pin Commander Spock's sentence and *still on the battlefield* as *be*'s locative
  Complement, and `central_adverb_position.txt` passes.

## Pointers

- `cargo xtask english parse "If that card is still on top of your library, draw a card."`
- `docs/decisions/english.md` §3, "A central Adjunct after *be*"
