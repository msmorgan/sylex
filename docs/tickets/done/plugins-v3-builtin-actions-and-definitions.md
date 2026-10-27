---
needs: [plugins-v3-builtin-keywords]
---
# Migrate `Definitions/Actions.lean`, leaf definitions, and rules into `plugins_v3/builtin/`

**Extract `Actions.lean`, `Counters.lean`, `Subtypes.lean`, `SubtypeRules.lean`,
`Designations.lean`, `DesignationRules.lean`, `Relations.lean`, `CardTypes.lean`,
`CardTypeRules.lean`, `Phrasings.lean`, and the `RegistryDefinition` conferral rows in
`Proofs/Rules.lean` into individual `.lean` files under `plugins_v3/builtin/`.** Third of four
atomic tickets migrating Lean declarations into `plugins_v3/builtin/`
([`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §§4.4–4.5). Standard
constraints apply.

## Scope

1. **Extract CR §701 keyword actions (`Definitions/Actions.lean`, ~827 lines):**
   - Move each keyword action `Definition` into
     `plugins_v3/builtin/definitions/keyword_action/<name>.lean` using the `action` command macro.
   - Re-spell `earthbend` (`[CR#701.66a]`) to reference `haste` directly and `timeTravel`
     (`[CR#701.56a]`) to reference `suspend` directly instead of `.named` string literals.
   - Omit manual `.ownScope`, `inCaller`, parameter `firstMention`/`laterMention`, and `theActor`
     `firstMention`/`laterMention` in `fight`, `explore`, `connive`, and `endure`.
   - Move the templates in `Definitions/Actions.lean` (`convert`, `destroyCantRegenerate`,
     `exileUntil`, `attachToIt`, `controllerSacrifices`, `discardThenForEachNonland`,
     `doesntUntap`, `mayDeclineUntap`, `untapDuring`, `mayBeAttachedTo`, `createTappedToken`,
     `createTappedAttacking`, and `searchLibraryOrGraveyard`) and
     `Definitions/Phrasings.lean` (~153 lines) into `plugins_v3/builtin/templates/<cat>/<name>.lean`.

2. **Unify and extract counters, subtypes, designations, relations, card types, and rules:**
   - Combine each counter in `Definitions/Counters.lean` with its conferrals from `Proofs/Rules.lean`
     into `plugins_v3/builtin/definitions/counter/<name>.lean` (`counter` command macro).
   - Combine each subtype in `Definitions/Subtypes.lean` with its rules from
     `Definitions/SubtypeRules.lean` into
     `plugins_v3/builtin/definitions/subtype/{creature,artifact,enchantment,land,planeswalker,battle,dungeon,spell}/<name>.lean`
     (`subtype` command macro).
   - Combine each designation in `Definitions/Designations.lean` with its rules from
     `Definitions/DesignationRules.lean` into `plugins_v3/builtin/definitions/designation/<name>.lean`
     (`designation` command macro).
   - Move `Definitions/Relations.lean` into `plugins_v3/builtin/definitions/relation/<name>.lean`
     (`relation` command macro).
   - Combine `Definitions/CardTypes.lean` and `Definitions/CardTypeRules.lean` into
     `plugins_v3/builtin/definitions/card_type/<name>.lean` (`card_type` command macro). Preserve the `CardType` structure and move the ten
     checker-level `CardType.<name>` constants and `cardTypes` into `Words.lean`.
   - Move standalone state-based action and conferral rows into `plugins_v3/builtin/rules/{sba,conferral}/<name>.lean`.

3. **Delete migrated `lean/Semantics/Definitions/*.lean` submodules:**
   - Remove the migrated `Definitions/*.lean` files and their imports from `lean/Semantics/Definitions.lean`.

## Proof

- `Definitions/Actions.lean`, `Counters.lean`, `Subtypes.lean`, `SubtypeRules.lean`,
  `Designations.lean`, `DesignationRules.lean`, `Relations.lean`, `CardTypes.lean`,
  `CardTypeRules.lean`, and `Phrasings.lean` are deleted and replaced by individual `.lean` files in `plugins_v3/builtin/`.
- `earthbend` references `haste` directly and `timeTravel` references `suspend` directly with zero
  `.named` workarounds.
- `cargo xtask lean check` (`lake build --wfail`) exits 0 with zero errors and zero warnings.

## Current declaration inventory and settled placement

The current declarations control the extraction; the examples above are not additions.
`Tokens.lean` is absent. The three characteristic-bundle definitions in `Actions.lean`
(`foodToken`, `treasureToken`, `clueToken`) remain characteristic-bundle helpers in
`templates/noun/`; the seven predefined-token proof fixtures remain in `Proofs/Rules.lean`.
`undercity` belongs in `definitions/subtype/dungeon/`.

`damageResult` and the named conferral helpers live in `templates/ability/` (returning
`Conferral`); `StaticSpec` helpers also live there. Condition, predicate, amount, action,
and noun helpers use their matching template categories. `partyRoles` returns a list of
predicates. Face-down and token characteristic descriptions use `templates/noun/`.
The registry rows from `Proofs/Rules.lean` are merged into their catalog declarations,
including Equipment, Fortification, Aura, Gargoyle and left-half-unlocked.

Regeneration shield creation remains a keyword action with `deed := none`: only shield
application carries the regenerate deed. Catalog values are registered before bundle rule
bodies so unified conferrals may refer back to their own catalog value through helpers;
rule bodies still undergo dependency ordering and generated checker proofs.

Lexical metadata comes from the retained RON catalogs. `strikeCounter` has no matching
counter catalog row; its current declaration is retained with its literal counter lemma.
Foretold and plotted likewise retain their current names where the designation catalog
has no row. Spellings use named angle-bracket parameters. Optional phrases, dependent
agreement and internal helpers with no standalone Oracle phrase carry `TODO(spelling)`
instead of a lossy string (Oracle style guide §§2, 5, 12 and 14).

| Directory under `plugins_v3/builtin/` | Extracted declarations |
| --- | ---: |
| `definitions/card_type/` | 10 |
| `definitions/counter/` | 218 |
| `definitions/designation/` | 21 |
| `definitions/keyword_action/` | 88 |
| `definitions/relation/` | 3 |
| `definitions/subtype/artifact/` | 22 |
| `definitions/subtype/battle/` | 1 |
| `definitions/subtype/creature/` | 324 |
| `definitions/subtype/dungeon/` | 1 |
| `definitions/subtype/enchantment/` | 13 |
| `definitions/subtype/land/` | 17 |
| `definitions/subtype/planeswalker/` | 80 |
| `definitions/subtype/spell/` | 5 |
| `rules/conferral/` | 5 |
| `rules/sba/` | 1 |
| `templates/ability/` | 27 |
| `templates/action/` | 10 |
| `templates/amount/` | 2 |
| `templates/condition/` | 6 |
| `templates/noun/` | 38 |
| `templates/predicate/` | 18 |
