---
needs: []
---

# Where designation shape, uniqueness and persistence live

## Why

Every designation has a storage shape, a uniqueness and a persistence, the three axes of
core's `DesignationDef::Stored` (`crates/core/src/designation.rs`). `.syl` designations state only
`scope` and, for the monarch and the initiative, `sole = true`, so a Ring-bearer's uniqueness for
each controller, day and night excluding each other, and saddled lasting until end of turn are
stated nowhere a reader of `.syl` can see. Either the declarations carry these values, because
something reading `.syl` needs them (holder eviction [CR#725.3,726.3,701.54a,310.9f]), or they are
engine-only state that lowering supplies. Where they live is the owner's open question.

The values, wherever they come to live:

| designation | shape | uniqueness | persistence |
|---|---|---|---|
| citysBlessing | Flag | None | Permanently |
| commander | Flag | None | Permanently |
| day | Flag | PerGame | Permanently |
| enduringStory | Flag | None | Permanently |
| foretold | Flag | None | ObjectLifetime |
| goaded | one instance per goader | None | EffectSupplied, per instance |
| harnessed | Flag | None | ObjectLifetime |
| initiative | Flag | PerGame | Permanently |
| leftHalfUnlocked | Flag | None | ObjectLifetime |
| level | Number | None | ObjectLifetime |
| monarch | Flag | PerGame | Permanently |
| monstrous | Flag | None | ObjectLifetime |
| night | Flag | PerGame | Permanently |
| plotted | Flag | None | ObjectLifetime |
| prepared | Flag | None | ObjectLifetime |
| protector | Player | one per battle | ObjectLifetime |
| renowned | Flag | None | ObjectLifetime |
| rightHalfUnlocked | Flag | None | ObjectLifetime |
| ringBearer | Flag | one per controller | until replaced or control changes |
| saddled | Flag | None | UntilEndOfTurn |
| solved | Flag | None | ObjectLifetime |
| suspected | Flag | None | ObjectLifetime |

Rows core's axes cannot state yet:

- `goaded`: a creature can be goaded by several players, and the same player goading it again has
  no effect [CR#701.15c,701.15d]. Each goader's instance has its own duration: a goad action's
  ends at that goader's next turn [CR#701.15a], and a static's ends when the static stops applying
  [CR#611.3a]. One flag with a goader set cannot express instances that end at different times.
  The CR does not say whether one player's second instance with a different duration exists
  (mtg-rules UD-21).
- `ringBearer`: "is your Ring-bearer" means on the battlefield under your control with the
  designation [CR#701.54e], and it ends when another creature becomes your Ring-bearer or another
  player gains control of it [CR#701.54a]. Its holder is always its controller, so it is a Flag
  with no player link. Neither its uniqueness (one for each controller) nor its persistence is a
  value of the current enums.
- `protector`: each battle has one player as its protector, chosen as it enters [CR#310.9,310.9a];
  another player becoming its protector ends the old one's [CR#310.9f], and state-based actions
  re-choose an absent or ineligible protector [CR#704.5x,704.5y]. Its value is a player. The corpus
  has no declaration for it.
- `DesignationShape::Relation` ("A relation to another object") has no user: no designation links
  two objects. It goes.
- `day` and `night` are two designations, and once the game has one it has exactly one of them
  from then on [CR#731.1]. PerGame uniqueness does not state that they exclude each other.
- `PerPlayer` meant "each player may hold one" for citysBlessing and enduringStory and "at most one
  object for each player" for ringBearer. Any number of players may hold the city's blessing or an
  enduring story [CR#702.131c,702.195b], so those rows are None; the per-controller limit is
  ringBearer's alone.
- `level` reads as 1 on a permanent without one [CR#716.2d]; Number has no default.

## Done when

- The owner has decided where each of the three values lives, and why.
- Every designation, `protector` included, states its values in that place, matching the table
  with the rows above resolved; `sole` is folded into uniqueness if uniqueness moves to `.syl`.
- If the values are on `.syl`, the `.syl` checks pass; if they are engine-side, the engine reads
  them from one declared place and `core-getdesignation-scopes-and-eviction` uses them.

## Pointers

`plugins/builtin/definitions/designation/`, `crates/core/src/designation.rs`,
`core-getdesignation-scopes-and-eviction`, `core-designation-card-scope`, the mtg-rules
`designations` map.
