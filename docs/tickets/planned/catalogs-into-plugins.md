---
needs: [semantics-v1-cutover, plural-name-flag]
---
# Move the Catalogs into `plugins/` declarations

## Why

Every game word is a `.syl` declaration under `plugins/`, its `lexical` field beside its spelling
and meaning (`docs/decisions/lexicon.md` §3). The Catalogs still live in
`crates/vocabulary/catalogs/*.ron`. Some are declared twice: card types, subtypes, counters,
designations, keyword abilities and keyword actions also have `plugins/builtin/definitions/`
declarations, so `ageCounter.syl` and the `age` entry of `counters.ron` both declare the age
counter. The rest have no `.syl` declaration at all: ability words, statuses, turn parts,
predefined tokens, cited names, Introduced Names, plural names, Anchor Words and vote options.

## Done when

- Every Catalog item is declared in `plugins/`, and `sylex_vocabulary` reads it from there.
- `crates/vocabulary/catalogs/` is gone, and `sylex_vocabulary` declares only Core English.
- Each Catalog with no declaration today gets a declaration sort, recorded in `lexicon.md` §3.
- No declaration names a specific card. The card lists in `introduced_names.ron`,
  `cited_names.ron`, `anchor_words.ron` and `vote_options.ron` are facts about cards known before
  parsing, and they go; such a name is found from the card's own text by rule, as a Shortened Name
  is [CR#201.5c], or admitted at parse time without a list. Which is chosen here, with its
  parse-forest cost measured.

## Open

`lexicon.md` §3 fixes only that each game word is a declaration of its own sort with a `lexical`
field (*Khans* is `khans.syl`, of the AnchorWord sort). Undecided:

- Expansion names a card's text cites (*Antiquities*) may need a set-name Catalog. A dungeon name
  (*Tomb of Annihilation*) is a Catalog item of its dungeon's definition; dungeons are not declared
  yet.

## Pointers

- `crates/vocabulary/catalogs/` and `crates/vocabulary/src/schema.rs`
- `plugins/builtin/definitions/` (the existing `lexical` blocks)
- `docs/decisions/lexicon.md` §3, `docs/decisions/syl.md` §1
- `docs/contexts/oracle-english/CONTEXT.md` (Catalog, Provenance)
