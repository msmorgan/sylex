---
needs: [syl-concepts-modeled-twice, syl-core-deeds-out-of-keyword-actions, syl-each-plural-referent, syl-numeric-anaphors, syl-printed-read-order, syl-pronoun-swallowing-templates, syl-spelling-conventions, syl-this-head-template, syl-zone-words]
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
  twinned by plurality where discourse should decide. Each one found is fixed in the
  checkpoint's own landing or ticketed, and the landing lists them.
