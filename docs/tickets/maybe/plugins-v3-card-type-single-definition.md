---
needs: [plugins-v3-card-class]
---
# [design] One definition per card type: `Words.lean` `semantic_definition CardType.*` vs `card_type` plugins

Each card type is declared twice, in two different shapes.

- `lean/Semantics/Words.lean:70-112` declares `semantic_definition CardType.creature` through
  `CardType.dungeon`, plus `cardTypes`. The checker reads these by value: `Check/Words.lean`
  uses `normalizeTypes`, `mergeTypes` and `commonTypes`, and `Check/Card.lean`,
  `Check/Queries.lean`, `Check/AbilityRules.lean` and about 60 proof files use them too.
- `plugins_v3/builtin/definitions/card_type/*.lean` declares `card_type` items that carry the
  same fields plus `confers`.

`lean/CONTRACTS.md:154-157` makes the core copies deliberate: card types are "the one catalog
below the stack". Moving `cardTypes` into `Semantics.Definitions` would create an import cycle.
`Definitions` imports the checker, and the checker needs the card types.

## Options

1. Keep the core catalog. Each `card_type` plugin then references its core value instead of
   restating `cardClass` and `stats`, and adds only `confers`. Update CONTRACTS.md to name the
   plugin as the place conferrals live.
2. Move the card types into the bundle and make the checker take the catalog as a parameter.
   This is larger, and it changes the contract.
3. Keep both copies, and pin that they are equal with a Lean theorem per card type.

## After the ruling

Whatever is chosen, CONTRACTS.md and `Proofs/Tables.lean` say so. The `/// Mirrors
Semantics/Words.lean:NN` comments in `deckmaste_semantics_v3/src/ast.rs` are refreshed if lines
move.
