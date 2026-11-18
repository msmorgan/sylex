---
needs: [v1-retirement, lexicon-dungeon-names, english-names-read-from-text, english-set-names-read-from-text, english-anchor-words-read-from-text, english-vote-options-read-from-text]
---
# Move the Catalogs into `plugins/` declarations

## Why

Every game word is a `.syl` declaration under `plugins/`, its `lexical` field beside its spelling
and meaning (`docs/design/lexicon.md` §3). The Catalogs still live in
`crates/vocabulary/catalogs/*.ron`. Some are declared twice: card types and supertypes (`types.ron`),
subtypes, designations, predefined tokens (`tokens.ron`), keyword abilities and keyword actions
also have `plugins/builtin/definitions/` declarations, so `flying.syl` and the `flying` entry of
`keyword_abilities.ron` both declare flying. The rest (`ability_words.ron`,
`statuses.ron`, `turn_parts.ron`) have no `.syl` declaration yet.

## Done when

- Every Catalog item is declared in `plugins/` (or generated from Oracle data for card and dungeon
  names per `lexicon.md` §3), and `sylex_vocabulary` reads it from there.
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
