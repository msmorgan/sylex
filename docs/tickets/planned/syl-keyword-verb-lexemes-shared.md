---
needs: []
---

# A verb two keyword phrases share is one lexeme outside both

## Why

A verb lexeme is owned by the catalog entry that declares it. *Face* is declared by the keyword
action Face a Villainous Choice [CR#701.55] (`crates/vocabulary/catalogs/keyword_actions.ron`),
so "Whenever you face a dilemma" [CR#700.17] reads *face* as that keyword action's lexeme, though
it is the same verb. *Beam* is declared inside the Beam me up keyword ability's entry
(`keyword_abilities.ron`) for "can't be beamed up" [CR#702.196b]. Neither verb means anything on
its own, and the loader refuses a Core verb that is also a Catalog item of the same Category, so
the verb can't simply be declared once in `verbs.ron`.

## Done when

- *Face* and *beam* are each declared once, in a place that both the keyword phrase and its other
  uses (`faceDilemma`, `beamUp`) name, and no keyword entry owns a verb another declaration uses.
- `cargo test -p sylex_vocabulary` and `cargo test -p sylex_english` pass, and the Seven of Nine,
  Marooned and Scotty units still read in `cargo xtask english strata`.

## Pointers

- `crates/vocabulary/catalogs/keyword_actions.ron`, `keyword_abilities.ron`
- `plugins/builtin/primitives/action/faceDilemma.syl`,
  `plugins/builtin/definitions/keyword_action/beamUp.syl`
- `docs/decisions/lexicon.md` §3 (Core vs Catalog)
