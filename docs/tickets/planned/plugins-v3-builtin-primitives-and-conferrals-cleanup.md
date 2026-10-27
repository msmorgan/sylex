---
needs: [plugins-v3-builtin-cleanup]
---
# Clean up `plugins_v3/builtin` conferral helpers, redundant primitive wrappers, and `Words.lean` duplicates

**Inline the 26 single-use rules-table conferral helpers into their owning declarations, eliminate
the 109 redundant type-prefixed primitive wrappers in `plugins_v3/builtin/primitives/`, rehome
composite phrasings from `primitives/` to `templates/`, and remove duplicate `CardType.*`
definitions from `lean/Semantics/Words.lean`.** Standard constraints apply.

## Scope

1. **Inline the 26 single-use conferral/saga/siege helpers from `plugins_v3/builtin/templates/`:**
   - During `plugins-v3-builtin-actions-and-definitions` (`umuslnvn`), 26 local `def`s from
     `CardTypeRules.lean`, `SubtypeRules.lean`, and `DesignationRules.lean` were extracted as
     standalone `template` files in `plugins_v3/builtin/templates/`:
     - `templates/ability/`: `toughnessZero`, `lethalDamage`, `loyaltyZero`, `battleDefenseZero`,
       `summoningSicknessAttack`, `summoningSicknessActivation`, `creatureDamageMarking`,
       `creatureCanAttack`, `creatureCanBlock`, `combatant`, `planeswalkerLoyalty`,
       `planeswalkerCanBeAttacked`, `planeswalkerLoyaltyDamage`, `battleDefense`,
       `battleCanBeAttacked`, `battleDefenseDamage`, `battleProtector`, `siegeProtector`,
       `siegeLastCounter`, `siegeDefenseZero`, `monarchDraw`, `monarchSteal`, `initiativeSteal`.
     - `templates/action/sagaLoreIncrement.lean`, `templates/condition/sagaHasChapterAbilities.lean`,
       `templates/condition/sagaHasTriggeredChapter.lean`,
       `templates/condition/siegeHasNoWaitingTrigger.lean`.
   - Inline these rules directly into `confers := [...]` on
     `plugins_v3/builtin/definitions/card_type/{creature,planeswalker,battle}.lean`,
     `plugins_v3/builtin/definitions/subtype/{battle/siege,enchantment/saga}.lean`, and
     `plugins_v3/builtin/definitions/designation/{monarch,initiative}.lean` (matching how
     `aura.lean`, `class.lean`, `case.lean`, and `spacecraft.lean` inline their `confers := [...]`),
     delete the 27 single-use template files, and update `lean/Semantics/Proofs/Rules.lean` to
     inspect `confersOf <decl>_definition` directly.

2. **Eliminate the 109 redundant type-prefixed primitive wrappers in `plugins_v3/builtin/primitives/`:**
   - During `plugins-v3-builtin-primitives-and-templates` (`uvnrnvqq`), when enforcing
     `primitive_boundary%`, 109 synthetic camelCase type-prefixed `primitive` wrappers were emitted
     (`abilityActivated.lean`, `actionMove.lean`, `actionChoose.lean`, `staticSpecCant.lean`,
     `staticSpecConjunction.lean`, `staticSpecRevealedAsCast.lean`, `moveRiderAttachedTo.lean`,
     `moveRiderNot.lean`, `conditionDid.lean`, `predicateDid.lean`, `nounPhrasePro.lean`, etc.)
     even where clean unprefixed primitives/templates (`activated.lean`, `move.lean`, `choose.lean`,
     `cant.lean`, `conjunction.lean`, `did.lean`, `pro.lean`) already existed or where the
     declaration name should be the bare constructor name (`revealedAsCast`, `attachedTo`, `not`,
     etc.) dispatched by `(bare_name, return_type)`.
   - Replace every call site of a redundant type-prefixed wrapper with the unprefixed primitive or
     template (renaming any wrapper that is the sole binding for its constructor to its bare
     constructor name) and delete the redundant wrapper files.

3. **Move composite phrasings out of `plugins_v3/builtin/primitives/` into `plugins_v3/builtin/templates/`:**
   - `plugins_v3/builtin/primitives/` is strictly for 1-to-1 AST constructor bindings
     (`docs/decisions/semantics-v3.md`). Composite or specialized phrasings that were placed in
     `primitives/` during extraction (`onceEachYourTurn`, `oncePerGame`, `oncePerObject`,
     `oncePerTurn`, `restOfGame`, `battlefield`, `graveyard`, `graveyardOf`, `hand`, `handOf`,
     `library`, `libraryOf`, `activatedBy`, `activatedOnlyDuring`, `activatedOnlyIf`,
     `activatedOnlyOnce`, `activatedOnlyOnceIf`, `when`, `whenever`, `at_`, `turnFaceUp`, `flip`,
     etc.) must be `template` declarations in `plugins_v3/builtin/templates/` composed over
     1-to-1 primitives.

4. **Remove duplicate `CardType.*` definitions from `lean/Semantics/Words.lean`:**
   - `lean/Semantics/Words.lean` still defines `semantic_definition CardType.creature` ..
     `CardType.dungeon` (`:70-115`) alongside the canonical `card_type` declarations in
     `plugins_v3/builtin/definitions/card_type/*.lean`.
   - Remove the duplicate `CardType.*` definitions from `lean/Semantics/Words.lean` (moving
     `cardTypes` to `Semantics.Definitions` after the bundle loads) so each card type is defined in
     exactly one place (`plugins_v3/builtin/definitions/card_type/*.lean`).

## Acceptance

- The 27 single-use conferral/saga/siege helper files in `plugins_v3/builtin/templates/` are deleted
  and their rules live inline in `confers := [...]` on their owning `card_type`, `subtype`, and
  `designation` declarations.
- Zero redundant `<typePrefix><Ctor>.lean` wrapper files remain in `plugins_v3/builtin/primitives/`.
- Every file in `plugins_v3/builtin/primitives/` is a 1-to-1 AST constructor binding; composite
  phrasings live in `plugins_v3/builtin/templates/` as `template` declarations.
- `CardType.creature` .. `CardType.dungeon` are defined only in
  `plugins_v3/builtin/definitions/card_type/*.lean`, not duplicated in `lean/Semantics/Words.lean`.
- `cargo test -p deckmaste_semantics_v3 -p deckmaste_lowering_v3 -p xtask`,
  `cargo clippy -p deckmaste_semantics_v3 -p deckmaste_lowering_v3 -p xtask --tests -- -D warnings`,
  `cargo xtask lean check`, and the CR citation gates pass.
