---
needs: []
---

# Which *of* phrases after a noun are modifiers?

## Why

An *of* phrase after a noun is either a Complement the noun's frame selects
(`Complement(PP(["of"]))` in `crates/vocabulary/lexicon/english/nouns.ron`) or a modifier that
`of`'s `(complement: NP, hosts: Nominal)` frame in `prepositions.ron` licenses on any Nominal.
Which it is decides which Readings exist wherever the phrase meets another post-head dependent
or a coordination, and what attaches to what in them. A noun whose meaning involves the other
entity (a property and its bearer, a relation and its other term, a part and its whole, a thing
and its natural source) takes the phrase as a Complement (CGEL, Ch. 5, §14.1, pp. 441–442); one
that lacks the frame leaves its *of* phrase to the modifier licence, which then reads a missed
Complement as a modifier. *Ability* takes its *of* phrase as a Complement: *Cursed Totem*'s
"Activated abilities of creatures" names the abilities' source [CR#113.7], and the phrase
alternates with a genitive (*a creature's abilities*), as Complements do (§16.5.3, p. 475).

## Done when

- Every *of* phrase the before-forest attaches as a Nominal's modifier is classified by its head
  noun as a genuine modifier (CGEL §14.2 [14i], p. 446: *a woman of great wisdom*, *a school of
  this type*) or a missed Complement, with the units that show each.
- Each noun with a missed Complement gets its `Complement(PP(["of"]))` frame, and the frame
  decision for *ability* is recorded with its CGEL citation where the frame is declared.
- The same test is applied to the other nouns in `nouns.ron` whose only frame is
  `Complement(PP(["of"]))` (the quantity and partitive nouns against CGEL Ch. 5 §3), and each
  that fails it is listed and changed, or ticketed.
- `of`'s modifier frame in `prepositions.ron` is narrowed to the genuine modifiers that remain,
  or removed if none remain; where it cannot be narrowed, the reason is stated where the frame
  is declared.
- The census shows which Readings changed on the supported corpus and why, and `cargo xtask
  english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass for every
  changed crate.

## Pointers

`crates/vocabulary/lexicon/english/nouns.ron`; `crates/vocabulary/lexicon/english/prepositions.ron`
(`of`); `crates/english/grammar/english.fug` (the noun complement productions and
`adjunct PrepositionPhrase { host: Nominal, … }`); `docs/design/english.md` (§1, "An Adjunct or
Modifier is selected by no head"); `crates/english/README.md`, "The forest database";
`cargo xtask english forestwalk --card "Cursed Totem" --line 1`.
