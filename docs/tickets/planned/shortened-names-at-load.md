---
needs: []
---

# Compute Shortened Names at load, not from a generated file

## Why

A legendary face's Shortened Name [CR#201.5c] comes from a rule (`short_name` in
`crates/catalogs/src/nicknames.rs`) plus an attestation check against that face's own Oracle text.
`cargo xtask catalogs` runs both over the whole corpus and writes the results to
`data/gen/catalogs/card-name-nicknames.json`, which `sylex_vocabulary` then looks up. Everything
the computation needs (the name, the type line, the text) is at hand when the face is loaded, so
the generated file is a cache with its own corpus hash and staleness check, and nothing gains from
it.

## Done when

- The lexicon computes each legendary face's Shortened Name when it loads that face: the same rule
  (comma prefix; none for names starting "The "; trailing Roman numeral dropped; earliest " and
  the ", " the " or " of "; first word; none when equal to the full name) and the same attestation
  check.
- `card-name-nicknames.json`, `CardNameNicknames`, its catalog output entry and `SHORTENED_NAMES`
  are gone.
- The rule's unit tests survive with the rule (including *King Darien XLVIII* and *Grand Arbiter
  Augustin IV*).
- The English census shows no change in which Shortened Names are in scope:
  `cargo xtask english strata --check`.
- `cargo test` and clippy pass for `sylex_vocabulary`, `sylex_catalogs` and `sylex_english`.

## Pointers

`crates/catalogs/src/nicknames.rs`, `crates/catalogs/src/io.rs`,
`crates/catalogs/tests/suite/nicknames.rs`, `crates/vocabulary/src/names.rs`,
`crates/vocabulary/src/lib.rs` (`SHORTENED_NAMES`), `crates/xtask/src/catalogs.rs`,
`docs/decisions/lexicon.md` §6.
