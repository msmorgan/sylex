---
needs: [syl-ability-words-once, syl-add-mana-symbols, syl-alternative-costs-one-model, syl-at-turn-point, syl-bodiless-keywords, syl-card-type-permanent-type, syl-cleave-bracketed-text, syl-condition-first-static, syl-local-binding, syl-lookback-point-from-trigger, syl-opaque-templates, syl-subtype-card-types, syl-token-definitions]
---

# Step 6 spine: the core constructs

## Why

The spine reshapes the core `.syl` constructs that most of the corpus is written in: card and
subtype types, tokens, keywords and ability words, Templates and bindings, events and turn points,
result reads, flags and alternative costs. Later stages write against it.

## Done when

- Every ticket in its `needs:` is in `done/`.
- A checkpoint pass over the spine's constructs (the card type, subtype, token, keyword,
  ability-word, Template, event, turn-point, result-read, flag and cost declarations under
  `plugins/builtin/`, and their kinds in `crates/xtask/src/dsl_check/schema.rs`) finds what is
  there only because Lean or its generator needed it: a slot or field that carries what the
  construct should say, a second declaration of one concept, a choice made by hand that the rules
  derive, or a validator exception kept for one file. Each one found is fixed in the
  checkpoint's own landing or ticketed, and the landing lists them.
