---
needs: [syl-union-membership, syl-clause-and-imperative]
specs: [kind-sorts-and-position-unions, voice-and-object-phrases]
---

# "Has …" is a verb phrase headed by "have"

## Why

The tests `hasCounter`, `hasKeyword`, `compare`, `shares` and `hasDesignation` are called after
"is" for wordings whose verb is "have", as are the two canon calls of `named` at `Predicative`: "has no ice counters on
it" (*Dark Depths*), "has two or fewer loyalty counters on him" (*Garruk Relentless*), "as though
it didn't have defender" (*Bristlepack Sentry*), "has the same name as" (*Candles of Leng*), "has
the city's blessing" (ascend), "has the chosen name". Each is unspelled or spelled as a different
phrase (the with-phrase of the same name), so the term does not say what the card prints.

## Open questions

- **What "have" takes.** The object is a noun phrase in every wording above ("no ice counters on
  it", "defender", "the same name as …"). Options: one verb `have(object: NounPhrase)`, the
  keyword, counters phrase and name each a noun phrase under it; or a verb per kind of object.
  Recommendation: one verb, since the wording is one.

## Done when

- The open question is answered in the declaration's doc comment.
- Each wording above is written as its card or rule prints it, and no test or `Predicative`
  declaration stands for a "has …" wording. `hasDesignation`'s "has the city's blessing" is among them; "you're the monarch" and
  "while saddled" are `syl-spelled-test-phrases`'s.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- "Have" with a participle ("can't have counters put on it"):
  `docs/specs/voice-and-object-phrases.md` §11.
- A continuous effect's "has flying": `syl-continuous-effect-subject`.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md` §2; `docs/specs/voice-and-object-phrases.md`;
the tests at `Test`; the two canon callers of `named` at `Predicative` (the declaration itself is
`syl-passive-participle-and-trigger-events`'s).
