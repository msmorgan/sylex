---
needs: [cleanup-step6-spine, syl-continuous-effect-subject]
specs: [voice-and-object-phrases]
---

# A printed "have" with a noun phrase and a bare verb phrase is `have`

## Why

Cards print "have" with a noun phrase and a bare verb phrase to say that the one named does the
inner phrase: *Dire Undercurrents*'s "you may have target player draw a card" and "you may have
target player discard a card". About 235 Vintage-legal cards print "you may have" before a noun
phrase (a regular expression over Oracle text, reminder text removed). `have` is the word for it
(spec §1), and `syl-clause-and-imperative` declares it with one caller, *Invigorate*'s cost.
Three canon cards still print the construction through a Template that fixes the whole sentence:
`mayHaveEnter`, "You may have $subject enter $riders", for *Clone*, *Quicksilver Gargantuan* and
*Sculpting Steel* ("You may have this creature enter as a copy of any creature on the
battlefield").

## Open questions

- **Whether `mayHaveEnter` is `have` over an "enters" phrase.** Its body is
  `replace(would(actor($subject), enter), optionally(withRiders($riders)))`: an optional
  replacement of the entering [CR#614.1c]. With the "enters" phrases standing at `VerbPhrase`
  (`syl-continuous-effect-subject`), the printed sentence is
  `clause(you, may(have(this(creature), …)))` over one, standing at `StaticSpec`. The spec says an
  "enters" phrase states a replacement effect with no "would" (§1) and does not say that it still
  does under `may(have(…))`. Options: the three cards write the clause, and lowering reads the
  optional replacement off it; or `mayHaveEnter` stays, a Template whose body is that clause.
  Recommendation: the cards write the clause and the Template goes, since its wording is the
  spec's `have` word for word.

## Done when

- The open question is answered.
- *Dire Undercurrents* is in `plugins/canon/cards/`, each ability's effect a
  `clause(you, may(have(target(…), …)))`, checked against its Oracle text.
- Every canon card that prints "have" with a noun phrase and a bare verb phrase writes `have`;
  the landing lists each with its Oracle line (*Clone*, *Invigorate*, *Quicksilver Gargantuan*
  and *Sculpting Steel* today, by a regular expression over the canon cards' Oracle text).
- `have`'s doc comment says who performs the inner phrase and cites a card.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- "Have" with a participle (*Melira's Keepers*'s "can't have counters put on it"): it waits on
  `english-have-past-participial-complement` and is not designed (spec §11).
- The keyword phrases "have" and "gain" ("can't have or gain flying"): not this construction.

## Pointers

- `docs/specs/voice-and-object-phrases.md` §1, §11
- `plugins/builtin/templates/ability/mayHaveEnter.syl`,
  `plugins/canon/cards/{Clone,Invigorate,Quicksilver Gargantuan,Sculpting Steel}.syl`
- `crates/english/judgments/causative_have.txt`
