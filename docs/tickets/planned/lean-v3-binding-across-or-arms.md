---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# A Template parameter mentioned in two arms of an `or` cannot be bound

Standard constraints apply. Split from `lean-v3-authoring-default-leaks` §3 (2026-10-25).

`objectsOfPlayer` (`plugins_v3/builtin/templates/predicate/objectsOfPlayer.syl`, "…that <player>
controls or owns but no player controls", its only caller *True-Name Nemesis*,
`plugins_v3/canon/cards/True-Name Nemesis.syl`) writes `player` twice, once in each arm of an
`.or`. ADR `docs/decisions/semantics-v3.md` §3.2 binds a Template
parameter mentioned more than once with `.firstMention`/`.laterMention`. Writing

```lean
semantic_template objectsOfPlayer (player : NounPhrase) : Predicate :=
  .or [ controlledBy (.firstMention "objectsOfPlayer.player" player),
        .and [ ownedBy (.laterMention "objectsOfPlayer.player" player),
               .not (controlledBy (a .isPlayer)) ] ]
```

made *True-Name Nemesis* fail the Lean card check with seven
`Refusal.bindingIdentity "objectsOfPlayer.player"`: the `.or` steps each arm from the common
header context, so the second arm's `.laterMention` cannot see the first arm's `.firstMention`.
`discourse`, ported with the same `.or` stepping, inherits the refusal.

Requirement on `discourse`: one Template parameter identity is shared across the disjuncts of an
`.or`. Decide how (for example, bind the parameter in the common header before the arms, or let
`laterMention` read a `firstMention` from a sibling disjunct), implement it in `discourse`, then
respell `objectsOfPlayer` (in `.syl`) as above.

Tests (`discourse`): *True-Name Nemesis* checks clean with the bound spelling; a `laterMention`
with no `firstMention` in any arm is still refused.
