---
needs: []
---
# *Among* takes a plural or coordinated Complement

## Why

*Among* selects a Noun Phrase, a keyword phrase or a slash measure of any Number, so *from among
first strike* reads with a single keyword. Aragorn, Company Leader (*your choice of a counter
from among first strike, vigilance, deathtouch, and lifelink*) therefore keeps a wrong Reading
beside the right one: *of [a counter from among first strike], [vigilance], [deathtouch], and
[lifelink]*, a coordination under *of* whose first conjunct ends in the one-keyword *among*
phrase. CGEL Ch. 7, p. 636: "The complement of *between* and *among* is required to be
semantically plural. Syntactically, it generally has the form of a plural NP (*between/among the
trees*) or an *and*-coordination of NPs"; it adds that *among* "readily accepts a collective
singular (*among the crowd*)", which no card prints.

## Design

- *Among*'s Complement is a plural Noun Phrase or a coordination, with any coordinator (*among
  one or two targets*; *among flying, first strike, lifelink, or vigilance*, Elspeth
  Resplendent). A collective singular is not admitted.

## Done when

- Every supported-corpus unit with *among* is recorded by the form of its Complement (plural
  Noun Phrase, coordination, other), from a parse and not a word count; a unit whose Complement
  is neither plural nor a coordination is reported before the rule is written.
- *Among* with a singular Noun Phrase, a single keyword or a single slash measure has no
  Reading, and the requirement is stated once, where a preposition's Complement requirements
  belong; its comment cites CGEL p. 636 and says a collective singular is not admitted.
- Aragorn, Company Leader loses the Reading above and keeps its correct one, pinned by a
  judgment on its text.
- The census shows no verdict change on the supported corpus, and every unit whose Readings
  change is checked for whether the lost Readings needed a singular *among* Complement.

## Pointers

- `crates/vocabulary/lexicon/english/prepositions.ron` (*among*),
  `crates/syntax/src/frames.rs` (`PrepositionFrame`).
- `crates/english/grammar/english.fug`: `category PrepositionPhrase`, `valence
  PrepositionPhrase`, `category KeywordPhrase`, `coordination KeywordPhrase`, `slot
  UnlikeObject`, `word Among`.
- `crates/english/judgments/README.md`.
