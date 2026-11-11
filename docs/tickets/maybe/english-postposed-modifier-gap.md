---
needs: []
---
# Postposed Object modifiers are built on an open Gap

## Why

`PostposedRelative`, `PostposedBareRelative` and `PostposedParticipialModifier` each match a
`VerbPhrase` with `head.Gap = NP`, then the Object, a predicate Adjunct and the postposed
modifier (*Choose a card at random that was exiled with ...*). The Object is supplied as the
filler of the head's open Noun Phrase Gap. A Verb Phrase passes an NP Gap up from places other
than its own Object position (a trailing Preposition Phrase's Complement, `GappedPurpose`), so
the Gap these Productions fill need not be the verb's Object, and they may admit orders such as
*[put into your hand] [a card] [at random] [that was exiled]*. `valence VerbPhrase` already
derives a postposed modifier for `PostposedAdjective` through its `postposed` field, without a
Gap.

## Open questions

- Derive a postposed relative or participial modifier through the valence's `postposed` path, as
  the equative adjective is, or keep the three Productions and restrict the Gap to the verb's
  own Object?

## Done when

- Whether the three Productions admit a Gap that is not the verb's Object is confirmed by
  parse, with the string that shows it. If they do not, the ticket closes with a comment on
  `PostposedRelative` saying why the Gap is always the Object's.
- A postposed modifier attaches only to the verb's own Object.
- *Choose a card at random that was exiled with ...* and *a card at random exiled with ...*
  keep a correct Reading, and the census shows no Reading lost on the supported corpus.

## Not in scope

- The `adjunct.ComplementKind = AdjP` requirement on two of the three:
  `english-single-card-productions`.

## Pointers

- `crates/english/grammar/english.fug`: `PostposedRelative`, `PostposedBareRelative`,
  `PostposedParticipialModifier`, `PostposedAdjective`, `category PostposedModifier`,
  `valence VerbPhrase` (`postposed`), `GappedPurpose`.
- `crates/syntax/src/compile.rs` (the `postposed` derivation).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/gemini.md` (C5).
