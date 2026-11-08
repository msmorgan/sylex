---
needs: []
---

# Where designation shape, uniqueness and persistence live

**Why.** Every designation has a storage shape, a uniqueness and a persistence, the three axes of
core's `DesignationDef::Stored` (`crates/core/src/designation.rs`). `.syl` designations state only
`scope` and, for the monarch and the initiative, `sole = true`, so a Ring-bearer's per-player
uniqueness, day and night being one value, and saddled lasting until end of turn are stated
nowhere a reader of `.syl` can see. Either the declarations carry these values, because something
reading `.syl` needs them (holder eviction [CR#725.3,726.3,701.54a]), or they are engine-only
state that lowering supplies. Where they live is the owner's open question.

The values, wherever they come to live:

| designation | shape | uniqueness | persistence |
|---|---|---|---|
| citysBlessing | Flag | PerPlayer | Permanently |
| commander | Flag | None | Permanently |
| day / night | Enum(day, night) | PerGame | Permanently |
| enduringStory | Flag | PerPlayer | Permanently |
| goaded | Relation | None | EffectSupplied |
| harnessed | Flag | None | ObjectLifetime |
| initiative | Flag | PerGame | Permanently |
| leftHalfUnlocked | Flag | None | ObjectLifetime |
| level | Number | None | ObjectLifetime |
| monarch | Flag | PerGame | Permanently |
| monstrous | Flag | None | ObjectLifetime |
| prepared | Flag | None | ObjectLifetime |
| renowned | Flag | None | ObjectLifetime |
| rightHalfUnlocked | Flag | None | ObjectLifetime |
| ringBearer | Relation | PerPlayer | EffectSupplied |
| saddled | Flag | None | UntilEndOfTurn |
| solved | Flag | None | ObjectLifetime |
| suspected | Flag | None | ObjectLifetime |

These rows are flagged as likely wrong:

- The single `day / night` Enum row contradicts the corpus, which declares `day.syl` and
  `night.syl` separately.
- `ringBearer` as EffectSupplied: a creature stays your Ring-bearer until another creature becomes
  your Ring-bearer or another player gains control of it [CR#701.54a], which is a rule-defined
  persistence, not one an effect supplies.
- `goaded` and `ringBearer` with shape `Relation`: each points at a player, goaded at the player
  or players who goaded it [CR#701.15b,701.15c] and the Ring-bearer at its player [CR#701.54a],
  while the glossary's Relation is a link between two Objects. Core's `DesignationShape::Relation`
  (documented as "A relation to another object") has the same defect. The replacement shape is
  open.

**Done when.**
- The owner has decided where each of the three values lives, and why.
- Every designation states its values in that place, matching the table with the flagged rows
  resolved and `foretold` and `plotted` filled in; `sole` is folded into uniqueness if uniqueness
  moves to `.syl`.
- If the values are on `.syl`, the `.syl` checks pass; if they are engine-side, the engine reads
  them from one declared place and `core-getdesignation-scopes-and-eviction` uses them.

**Pointers.** `plugins/builtin/definitions/designation/`, `crates/core/src/designation.rs`,
`core-getdesignation-scopes-and-eviction`, `core-designation-card-scope`.
