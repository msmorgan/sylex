---
needs: [plugins-v3-builtin-cleanup]
---
# Inline the 27 single-use conferral helpers into their owning declarations

Standard constraints apply. `plugins-v3-builtin-actions-and-definitions` (`umuslnvn`) turned
the local `def`s of `CardTypeRules.lean`, `SubtypeRules.lean` and `DesignationRules.lean` into
standalone templates. They are not phrasings ("Template" in
`docs/contexts/game-model/CONTEXT.md`). Each one belongs inside the `confers := [...]` of the
single declaration that uses it. `subtype/enchantment/aura.lean` is the model to follow; it is
the only declaration that already inlines its conferrals.

## The 27 files

- **`templates/ability/`, 23 files:**
  - Creature (owned by `card_type/creature.lean`): `toughnessZero`, `lethalDamage`,
    `summoningSicknessAttack`, `summoningSicknessActivation`, `creatureDamageMarking`,
    `creatureCanAttack`, `creatureCanBlock`, `combatant`.
  - Planeswalker (owned by `planeswalker.lean`): `loyaltyZero`, `planeswalkerLoyalty`,
    `planeswalkerCanBeAttacked`, `planeswalkerLoyaltyDamage`.
  - Battle (owned by `battle.lean`): `battleDefenseZero`, `battleDefense`,
    `battleCanBeAttacked`, `battleDefenseDamage`, `battleProtector`.
  - Siege (owned by `subtype/battle/siege.lean`): `siegeProtector`, `siegeLastCounter`,
    `siegeDefenseZero`.
  - Designations: `monarchDraw` and `monarchSteal` (owned by `designation/monarch.lean`),
    `initiativeSteal` (owned by `initiative.lean`).
- **Saga, 3 files:** `templates/action/sagaLoreIncrement.lean`,
  `templates/condition/sagaHasChapterAbilities.lean` and
  `templates/condition/sagaHasTriggeredChapter.lean`, all owned by
  `subtype/enchantment/saga.lean`.
- **Siege guard, 1 file:** `templates/condition/siegeHasNoWaitingTrigger.lean`. It is used only
  inside `siegeDefenseZero`, so it is inlined into that conferral, not into `siege.lean`
  directly.

`sagaHasChapterAbilities` is used twice in `saga.lean`. Bind it once in the declaration body
rather than duplicating it. `combatant` the helper is not `StaticSpec.combatant` the
constructor; only the helper goes.

## Proofs

`lean/Semantics/Proofs/Rules.lean` quantifies over all 27 names (`:156-374`). Rewrite each
theorem over `confersOf <decl>_definition` (`:143`). Three theorems need more than a membership
rewrite:

- `sagaLoreIncrementIsTurnBased` matches `doIf sagaHasChapterAbilities sagaLoreIncrement`
  structurally.
- `okSagaHasTriggeredChapter` and `okSiegeGuard` call `Condition.check` on the helpers
  directly.

State each one over the inlined term, extracted from the owning definition. Do not keep a
helper alive for the proof's sake.

Tickets that cite these helpers by name and must be re-pointed when they are claimed:
`lean-marked-damage-projection` (`lethalDamage`), `lean-defense-projection`
(`planeswalkerLoyalty`, `battleDefense`) and `lean-v3-venture-dungeon` (`initiativeSteal`).

## Acceptance

- The 27 files are deleted, and the owning declarations' conferral expansions are unchanged
  (baseline: only the 27 template keys are removed).
- `cargo xtask lean check`. Run tests for `deckmaste_semantics_v3` and `xtask`.

## Expansion baseline workflow

Save before editing; this single invocation includes builtin and canon, with canon
expanded over builtin:

```bash
cargo xtask expansions --out docs/evidence/plugins-v3-expansions/plugins-v3-conferral-inlining/before
```

After inlining, accept only the 27 removed helpers; owning registry expansions
and canon must remain unchanged. No rename map is needed:

```bash
cargo xtask expansions --compare docs/evidence/plugins-v3-expansions/plugins-v3-conferral-inlining/before \
  --expect 'toughnessZero : Conferral' \
  --expect 'lethalDamage : Conferral' \
  --expect 'summoningSicknessAttack : Conferral' \
  --expect 'summoningSicknessActivation : Conferral' \
  --expect 'creatureDamageMarking : Conferral' \
  --expect 'creatureCanAttack : Conferral' \
  --expect 'creatureCanBlock : Conferral' \
  --expect 'combatant : Conferral' \
  --expect 'loyaltyZero : Conferral' \
  --expect 'planeswalkerLoyalty : Conferral' \
  --expect 'planeswalkerCanBeAttacked : Conferral' \
  --expect 'planeswalkerLoyaltyDamage : Conferral' \
  --expect 'battleDefenseZero : Conferral' \
  --expect 'battleDefense : Conferral' \
  --expect 'battleCanBeAttacked : Conferral' \
  --expect 'battleDefenseDamage : Conferral' \
  --expect 'battleProtector : Conferral' \
  --expect 'siegeProtector : Conferral' \
  --expect 'siegeLastCounter : Conferral' \
  --expect 'siegeDefenseZero : Conferral' \
  --expect 'monarchDraw : Conferral' \
  --expect 'monarchSteal : Conferral' \
  --expect 'initiativeSteal : Conferral' \
  --expect 'sagaLoreIncrement : Action' \
  --expect 'sagaHasChapterAbilities : Condition' \
  --expect 'sagaHasTriggeredChapter : Condition' \
  --expect 'siegeHasNoWaitingTrigger : Condition'
```

The saved format is `before/snapshot.json` (version 2). Save commands refuse an
existing snapshot unless `--force` is deliberately added; unrelated nonempty
directories are always refused. Comparison writes nothing unless `--out` is also
given. Review the sampling summary for constructor omissions and capped products.
