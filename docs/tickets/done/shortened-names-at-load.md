---
needs: []
---

# Compute Shortened Names at load, not from a generated file

## Why

A legendary face's Shortened Name [CR#201.5c] comes from a rule plus an attestation check against
that face's own Oracle text. Generating both into `data/gen/catalogs/card-name-nicknames.json`
made the file a cache with its own corpus hash and staleness check. The rule needs only a face's
name, supertypes and types, so it is a public function a `.syl` face can call; the attestation
needs the whole corpus (a candidate must be used outside any complete card title), so it is
computed from the corpus when the lexicon loads.

## Done when

- `sylex_vocabulary::short_name(name, supertypes, types)` gives a face's Shortened Name by the
  rule: legendary permanents only; comma prefix; none for names starting "The "; trailing Roman
  numeral dropped; earliest " and the ", " the " or " of "; first word; none when equal to the
  full name. It needs no corpus, type line or Oracle text.
- The lexicon computes the attestation from the corpus when it loads (`AttestedShortenings`), and
  calls `short_name` for the rule.
- `card-name-nicknames.json`, `CardNameNicknames`, its catalog output entry and `SHORTENED_NAMES`
  are gone.
- The rule's unit tests survive with the rule (including *King Darien XLVIII* and *Grand Arbiter
  Augustin IV*).
- The English census shows no change in which Shortened Names are in scope:
  `cargo xtask english strata --check`.
- `cargo test` and clippy pass for `sylex_vocabulary`, `sylex_catalogs` and `sylex_english`.

## Pointers

`crates/vocabulary/src/shortening.rs`, `crates/vocabulary/src/names.rs`,
`crates/vocabulary/src/lib.rs`, `crates/catalogs/src/io.rs`, `crates/xtask/src/catalogs.rs`,
`docs/decisions/lexicon.md` §6.
