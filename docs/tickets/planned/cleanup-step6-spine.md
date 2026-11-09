---
needs: [syl-ability-words-once, syl-alternative-costs-one-model, syl-at-turn-point, syl-bodiless-keywords, syl-card-type-permanent-type, syl-cleave-bracketed-text, syl-condition-first-static, syl-local-binding, syl-lookback-point-from-trigger, syl-opaque-templates, syl-subtype-card-types, syl-token-definitions]
---

# Step 6 spine: the core constructs

## Why

The spine reshapes the core `.syl` constructs that most of the corpus is written in: card and
subtype types, tokens, keywords and ability words, Templates and bindings, events and turn points,
result reads, flags and alternative costs. Later stages write against it.

## Done when

- Every ticket in its `needs:` is in `done/`.
