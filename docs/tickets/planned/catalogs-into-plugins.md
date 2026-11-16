---
needs: [v1-retirement, lexicon-dungeon-names, english-names-read-from-text, english-set-names-read-from-text, english-anchor-words-read-from-text, english-vote-options-read-from-text]
---
# Move the Catalogs into `plugins/` declarations

## Why

Every game word is a `.syl` declaration under `plugins/`, its `lexical` field beside its spelling
and meaning (`docs/design/lexicon.md` §3). The Catalogs still live in
`crates/vocabulary/catalogs/*.ron`. Some are declared twice: card types, subtypes, counters,
designations, keyword abilities and keyword actions also have `plugins/builtin/definitions/`
declarations, so `ageCounter.syl` and the `age` entry of `counters.ron` both declare the age
counter. Others (`ability_words.ron`, `statuses.ron`, `turn_parts.ron`, `tokens.ron`,
`plural_names.ron`) have no `.syl` declaration yet, while the card-scoped name lists
(`introduced_names.ron`, `cited_names.ron`, `anchor_words.ron`, `vote_options.ron`) are retired by
their `*-read-from-text` tickets.

## Done when

- Every remaining Catalog item is declared in `plugins/` (or generated from Oracle data for card
  and dungeon names per `lexicon.md` §3), and `sylex_vocabulary` reads it from there.
- `crates/vocabulary/catalogs/` is gone, and `sylex_vocabulary` declares only Core English.
- Each Catalog with no declaration today gets a declaration sort, recorded in `lexicon.md` §3, and
  no declaration names a specific card.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo xtask english strata --check` and
  `cargo test` pass for all affected crates.

## Pointers

- `crates/vocabulary/catalogs/` and `crates/vocabulary/src/schema.rs`
- `plugins/builtin/definitions/` (the existing `lexical` blocks)
- `docs/design/lexicon.md` §3, `docs/design/syl.md` §1
- `docs/contexts/oracle-english/CONTEXT.md` (Catalog, Provenance)
