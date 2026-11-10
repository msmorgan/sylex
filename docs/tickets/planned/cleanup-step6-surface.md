---
needs: [syl-become-replaces-types, syl-concepts-modeled-twice, syl-each-plural-referent, syl-lean-doc-comments, syl-list-element-printed-order, syl-mana-for-each-template, syl-numeric-anaphors, syl-printed-read-order, syl-pronoun-swallowing-templates, syl-pt-change-sort, syl-spelling-conventions, syl-tail-placement, syl-this-head-template, syl-trigger-condition-written-form, syl-zone-words]
---

# Step 6 surface: how `.syl` reads

## Why

The surface stage settles how `.syl` reads: argument naming, declaration names, spellings,
printed order, self-references, and the words for zones, plural referents and numeric anaphors.
Later stages write against it.

## Done when

- Every ticket in its `needs:` is in `done/`.
- A checkpoint pass over how `plugins/` reads (declaration and file names, `spelling` strings and
  `TODO(spelling)` markers, argument naming and order, self-references, zone words, plural and
  numeric referents) finds what is still shaped by Lean: a name bent around a Lean keyword, a
  spelling or doc comment that carries generated text, a pronoun resolved by hand, or a Template
  that writes an anaphor inside it and so fixes a number its call site should print. Number
  itself stays in the tree as `Plurality`, and the closed-class pronoun pairs (`it`/`them`,
  `that`/`those` and the others) are kept. Each one found is fixed in the
  checkpoint's own landing or ticketed, and the landing lists them.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.
