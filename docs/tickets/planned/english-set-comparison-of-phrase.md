---
needs: []
---

# The *of* phrase of *the top card of your library*

## Why

In *Look at the top three cards of your library*, *of your library* names the pile whose cards
*top* ranks, and the grammar attaches it as a modifier of *three cards* (`of`'s
`(complement: NP, hosts: Nominal)` frame), as it would any descriptive *of* phrase. *Card* does not
license it: its meaning involves no library (CGEL, Ch. 5, §14.1, p. 442), and in the pool *card of
\<zone\>* never occurs without an ordering word before the noun (*top*, *bottom*, *first*, *the top
X plus one cards of your library*), while *a card of your choice* does. That is CGEL's *of* phrase
"indicating the set whose members are compared" after a superlative (*Kim wrote the best essay of
them all*; Ch. 13, §6.3.4, p. 1169), an indirect Complement licensed by the pre-head dependent
rather than the head (Ch. 5, §14.1, p. 443), as `ComparativeIndirectComplement` licenses *than*.
About 1,170 units have it (*top card of your library* ×405, *N cards of your library* ×593 in the
forest of `default`), so its analysis decides what most Readings with a library say.

## Done when

- *top*, *bottom* and the ordinals that rank a pile (*first*, *the first card of their program*)
  license the set's *of* phrase as an indirect Complement (CGEL citations in the production's
  comment), and it is no longer also a modifier of the same Nominal.
- The units that print *card(s) of \<zone\>* read it that way; the census shows which Readings
  changed and why, and `cargo xtask english strata --check`, `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for every changed crate.

## Pointers

`crates/english/grammar/english.fug` (`ComparativeIndirectComplement`, `ModifiedQuantity`);
`crates/vocabulary/lexicon/english/adjectives.ron` (*top*, *bottom*);
`cargo xtask english forestwalk --card "Lantern of Revealing" --line 2`.
