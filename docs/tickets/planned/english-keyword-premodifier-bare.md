---
needs: []
---
# A keyword that carries a parameter premodifies a noun

## Why

`KeywordPremodifier` takes any keyword phrase whose last parameter is no cost and no em-dash
ability, so a keyword with an ordinary parameter premodifies a noun: on Mirror Golem (*This
creature has protection from each of the exiled card's card types.*) one Reading takes
*[protection from each of the exiled card's]* as a premodifier of *card types*. Nine units have
such a Reading beside their correct one (Mirror Golem, Samite Elder, Opal Titan, Éowyn,
Fearless Knight, Haktos the Unscarred, Lavabrink Venturer, Rebbec, Architect of Ascension,
Spare from Evil, Devoted Caretaker), and no supported-corpus unit has a keyword with a
parameter as a correct premodifier: every correct one is bare (*a flashback cost*, *a flying
counter*, *ward abilities*). `feature KeywordForm = Bare | Parametric` is commented as telling
the two apart, but it is the keyword lexeme's (whether the keyword declares any parameter), so
bare *flashback* is `Parametric`.

## Done when

- `KeywordForm` says whether the phrase carries a parameter, valued where the phrase is built;
  its comment matches.
- `KeywordPremodifier`, `QuotedKeywordPremodifier` and `KeywordBase` take a keyword phrase
  that carries none; a keyword coordination (*eternalize or embalm ability*, Vizier of the
  Anointed) keeps its Reading; the `FinalParameter` exclusions on the three go if they then
  exclude nothing.
- Every other declaration that reads `KeywordForm` is checked against its new meaning.
- A judgment on Mirror Golem's text asserts against the premodifier Reading.
- The census shows no verdict change on the supported corpus, and every unit whose Readings
  change is checked; `cargo test` and `cargo clippy --tests -- -D warnings` pass for each
  changed crate.

## Not in scope

- A bare keyword read as a premodifier of its own parameter (*crew [2]*, *equip [{2}]*).

## Pointers

- `crates/english/grammar/english.fug`: `feature KeywordForm`, `feature FinalParameter`,
  `KeywordPremodifier`, `QuotedKeywordPremodifier`, `KeywordBase`, `BareKeywordMention`,
  `CompleteKeywordPhrase`, `KeywordInstruction`.
- `crates/syntax/src/compile.rs` (keyword parameter patterns).
