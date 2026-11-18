---
needs: []
---

# A noun that selects *of* admits a bare nominal or gerund oblique

## Why

A noun frame `Complement(PP(["of"]))` admits every Complement kind `of` takes as
`ComplementOnly`, including a bare count nominal and a gerund-participial clause. So *the number of
creature cards* also reads as *the number of [creature] cards*, with bare *creature* the oblique
(52 units with *number*), *the number of attacking creatures* as a gerund oblique (25 units), and
*loyalty abilities of Kasmina planeswalkers* as *abilities of [Kasmina]* (7 units): second trees
of one analysis, or ungrammatical ones. Only *end* (*end of turn*), *kind* (*each kind of
counter*; CGEL, Ch. 5, §3.4, p. 352), *part* (*as part of casting a spell*) and *instead* (*instead
of putting it into its owner's graveyard*) need those kinds. Marking `of`'s Nominal and Gerund
frames `KindSelected` and giving those four `PPWith(["of"], Nominal)` or `PPWith(["of"], Gerund)`
removes the spurious trees, but where two conjuncts share the *of* phrase by Right Node Raising,
each of *kind*'s two *of* frames gives the shared phrase its own gapped tree, whatever the shared
oblique's kind (*the number and kind of counters*, Tawnos's Coffin line 2: 330 Derivations to
2,970).

## Done when

- Under a head that selects *of* alone, an *of* phrase's oblique is a Noun Phrase, an Amount or a
  keyword phrase, never a bare nominal or a gerund; the heads that take those name them.
- A Right Node Raised *of* phrase is shared by each conjunct's frame that selects its oblique's
  kind, and only by those.
- The census shows which Readings changed and why, and `cargo xtask english strata --check`,
  `cargo test` and `cargo clippy --tests -- -D warnings` pass for every changed crate.

## Pointers

`crates/vocabulary/lexicon/english/prepositions.ron` (`of`, `instead`);
`crates/vocabulary/lexicon/english/nouns.ron` (*end*, *kind*, *part*, *number*);
`crates/syntax/src/compile.rs` (the `gap0` noun Complement variants);
`cargo xtask english forestwalk --card "Tawnos's Coffin" --line 2`.
