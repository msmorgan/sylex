---
needs: [engine-counters-api]
---
# Engine: player speed, start your engines! and max speed

## Why

Speed is a number a player has, not a counter. A player has no speed until a rule or effect sets
one [CR#702.179b], and an effect that reads the speed of a player with none sees 0 [CR#702.179f].
`PlayerState` (`crates/engine/src/player.rs`) has no speed, so none of these cards work:

- start your engines!: a player who controls a permanent with it and has no speed gets speed 1, as
  a state-based action [CR#702.179a,704.5aa];
- increasing the speed of a player who has none sets it to the amount [CR#702.179c];
- every player with speed 1 or more has a sourceless triggered ability that they control:
  "Whenever one or more opponents lose life during your turn, if your speed is less than 4, your
  speed increases by 1. This ability triggers only once each turn." [CR#702.179d];
- max speed: a player whose speed is 4 has max speed [CR#702.179e], and "Max speed — [ability]"
  grants that ability while its controller's speed is 4 [CR#702.178a].

## Done when

- `sylex_engine` tests with real start your engines! cards cover: getting speed 1 from the
  state-based action; speed rising at most once per turn, only on the controller's own turn, and
  never above 4; a max speed ability working only at 4; and a player with no speed reading as 0.

## Pointers

- `plugins/builtin/definitions/keyword_ability/startYourEngines.syl`, `maxSpeed.syl`, and
  `plugins/builtin/templates/quantity/speedOf.syl`: the `.syl` side. Its missing actions and
  state-based row are `lean-v3-player-speed-rules`.
- `crates/engine/src/player.rs`, `crates/engine/src/sba.rs`.
