---
needs: []
---
# `Stat::Loyalty` and `Stat::Defense` read counters on the battlefield and the printed number elsewhere

## Why

A planeswalker's loyalty is the printed number off the battlefield [CR#306.5a] and the number of
loyalty counters on it on the battlefield [CR#306.5c]; a battle's defense works the same way
[CR#310.4a,310.4c]. The engine reads `Stat::Loyalty` as the printed number in every zone and
`Stat::Defense` as the `DefenseCounter` count in every zone. So "damage equal to its loyalty"
(Compel Brutality) reads the printed loyalty of a planeswalker on the battlefield, and a battle's
defense is 0 off the battlefield. The test `stat_of_loyalty_reads_printed_loyalty` asserts the
wrong rule: a battlefield planeswalker printed 4 with one loyalty counter reads 4.

## Done when

- Both stats are zone-dependent in every read: on the battlefield the counter count, elsewhere
  the printed number. A snapshot reads the value in the zone the object left.
- A planeswalker enters with loyalty counters equal to its printed loyalty [CR#306.5b], and a
  battle with defense counters equal to its printed defense [CR#310.4b], although the
  battlefield read is now the counter count.
- `stat_of_loyalty_reads_printed_loyalty` is rewritten: 1 on the battlefield, 4 in hand.
- The doc comments that call loyalty and defense "layer-stable base state" (`layer.rs`) or
  `Stat::Loyalty` the printed number are corrected.
- Tests: the rewritten one; a planeswalker entering with counters equal to its printed loyalty;
  a battle entering with defense counters equal to its printed defense and reading its printed
  defense off the battlefield.

## Pointers

- `crates/engine/src/layer.rs`: `eval_stat_of` and the second `Stat::Loyalty`/`Stat::Defense`
  arm; `crates/engine/src/resolve/count.rs` (live and snapshot arms, the test);
  `crates/engine/src/trigger.rs` (snapshot arm); `crates/engine/src/activate.rs`;
  `crates/engine/src/target.rs`.
- The entry grants the engine loads today: `v1/plugins/builtin/rules/grant/planeswalker-loyalty.ron`
  (reads `StatOf(This, Loyalty)`); there is no battle entry grant there, and
  `v1/plugins/builtin/rules/sba/battle-defense-zero.ron` reads `StatOf(This, Defense)`.
- The `.syl` types confer both grants as `entersWithCounters(this, statOf(stat(…), this), …)`:
  `plugins/builtin/definitions/card_type/planeswalker.syl`, `battle.syl`.
