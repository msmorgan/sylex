---
needs: []
---

# `english parse` loads the corpus only for a named card

## Why

`cargo xtask english parse` calls `Corpus::load(|_| true)` on every call, keeping every supported
face of `data/scryfall/oracle-cards.jsonl` (211 MB), although the corpus serves only to build the
`CardScopes` a `<card=…>` entry is read with. An entry without `<card=…>` reads with the empty
scope (`CardScopes::scope`) and needs no corpus at all. Measured in the dev profile, a
call takes about 2.7 s, of which reading the snapshot is about 0.8 s (`english strata` takes
2.65 s over the full snapshot and 1.89 s over an empty one). `english strata --card` filters the
faces it keeps, but `Corpus::read` still parses every record before the filter sees it, so a
filtered load alone saves only the faces kept, not the read.

## Done when

- A `parse` call whose inputs name no `<card=…>` reads no corpus.
- A call with `<card=…>` inputs keeps only the faces whose name one of them names, so
  `CardScopes` sees every face of that name (the *Lightning Bolt* case in `judgment.rs`'s tests)
  and an unknown name is still the error "names no face of the supported corpus".
- The landing reports the wall time of one `parse` call without a card and one with a card,
  before and after, each the median of several runs in the same profile.
- The `english.rs` tests pass unchanged in what they assert; `cargo test -p xtask` and
  `cargo clippy -p xtask --tests -- -D warnings` pass.

## Open questions

- Whether a call with `<card=…>` also skips parsing records whose card it cannot be, so that it
  does not read the whole 211 MB stream:
  - keep `Corpus::read` as it is and filter faces after parsing, as `strata --card` does (about
    0.8 s per call remains);
  - a cheap test on each raw record before parsing it (the name occurs in the line), parsing only
    the lines that pass; a face name inside `card_faces` and JSON escapes in names must still
    match;
  - an index of the snapshot by face name, written once per snapshot.

## Pointers

- `crates/xtask/src/english.rs`: `parse`, `strata`.
- `crates/english/src/corpus.rs`: `Corpus::read`, `Corpus::load`, `Face`.
- `crates/english/src/judgment.rs`: `CardScopes`.
- `crates/upstream/src/scryfall.rs`: `oracle_cards`, `OracleCardReader`.
