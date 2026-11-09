---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, lean-retirement]
---
# A Template parameter mentioned in two arms of an `or` shares one binding

## Why

`objectsOfPlayer` (`plugins/builtin/templates/predicate/objectsOfPlayer.syl`, "protection from
<player>" [CR#702.16k]) writes `$player` once in each arm of an `or`: each object `player`
controls, or each object `player` owns that no player controls. Its one caller is *True-Name
Nemesis*. A Template that mentions a NounPhrase parameter more than once binds it at the first
occurrence, and the binding is generated (`docs/decisions/syl.md` §4). But reference resolution
steps each disjunct from the context before the `or`, so the second arm's mention cannot see the
binding the first arm made, and the parameter is refused or bound twice.

## Done when

- Reference resolution shares one Template parameter's Binding Identity across the disjuncts of
  an `or`; how (bind it before the arms, or let a later arm read the earlier arm's binding) is
  part of the reference-resolution design (`docs/decisions/semantics.md` §2).
- Tests in reference resolution, which does not exist yet: *True-Name Nemesis* checks, with `the(isChosenPlayer)`
  bound once for both arms of `objectsOfPlayer`; an anaphor in one arm that has no antecedent in
  any arm or before the `or` is still refused.

## Pointers

- `plugins/builtin/templates/predicate/objectsOfPlayer.syl`, `plugins/canon/cards/True-Name Nemesis.syl`
- `docs/decisions/syl.md` §4; `docs/decisions/semantics.md` §2.2 (boxes), §2.4
- Glossary: **Binding Identity** (`docs/contexts/game-model/CONTEXT.md`)
