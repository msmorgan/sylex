---
needs: []
---
# Move the engine-defined combat keywords into `.syl`

## Why

First strike, double strike, deathtouch, trample and banding change how combat damage is assigned
and dealt. The engine defines them (deathtouch's destruction rule [CR#704.5h] is already a `.syl`
row). `.syl` declares them as bodiless keyword abilities today and is to declare them as
primitives, so the engine's ownership is explicit. Each can become a Definition once `.syl` has
the primitives its rules need. In order of difficulty:

1. **Deathtouch and trample, together.** Trample's excess is what remains once the blockers are
   assigned lethal damage [CR#702.19b], and deathtouch makes any nonzero amount lethal for that
   purpose [CR#702.2c]. Neither can move alone. Trample may be a damage result.
2. **First strike and double strike.** They add a combat damage step [CR#510.4,702.7b,702.4b],
   which needs a turn-part primitive.
3. **Banding and "bands with other".** A block spreads to the whole band [CR#702.22h], and banding
   swaps who assigns combat damage [CR#702.22j,702.22k]. Raging River also regroups blockers, so
   the two may share primitives.

## Done when

- Each moved keyword is a Definition whose body states its rule, its primitive is deleted, and
  the engine's special case for it is gone.
- The `.syl` checks pass, and the engine's combat tests pass unchanged.

## Pointers

- `plugins/builtin/definitions/keyword_ability/` (`firstStrike`, `doubleStrike`, `trample`,
  `banding`, `bandsWithOther`, `deathtouch`)
- `docs/tickets/planned/lean-marked-damage-projection.md`: deathtouch's destruction rule
  [CR#704.5h], which is already `.syl`.
