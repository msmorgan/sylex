---
needs: [engine-statof-reference-channel]
---

# Engine: a value read over an illegal target or a missing reference doesn't happen, instead of reading 0

## Why

An illegal target reads as no object while its spell or ability resolves. If part of the effect
requires information about an illegal target, it fails to determine that information, and the
part that requires it doesn't happen [CR#608.2b]. `engine-statof-reference-channel` gives
`Count::StatOf` an "unavailable" result for this. The other reads in
`crates/engine/src/resolve/count.rs` that take a reference still read 0 when it names no object or
player:

- `Count::PlayerStatOf`, `Count::Opponents`, `Count::ManaAvailable` (`floated_mana`) and
  `Count::ManaAvailableKind`: `eval_player_ref(..).map_or(0, ..)`.
- `Count::CounterCount`: with no live object and no snapshot, `unwrap_or(0)`. This conflates a
  missing object with an absent counter kind on a real one.
- `Count::GreatestWatchedThreshold`: a missing object reads 0.
- `Count::TargetsOf`: a reference that isn't on the stack reads 0.
- `Countable::ManaSymbols` and `Countable::Singleton`: a stale or absent reference reads 0.
- `Count::Damage` panics instead: `self.objects.obj(id)` on the null id an illegal target reads
  as.

Canon's Axis of Mortality ("you may have two target players exchange life totals") is a witness.
Its body is `ChangeLife(Target(0), Set(PlayerStatOf(Target(1), Life)))` and its mirror. If one
target becomes illegal, the legal player's life is set to 0. Its ruling says that "the exchange
won't happen. Neither player's life total will change", which agrees with [CR#608.2b] and with an
exchange that can't be completed [CR#701.12a].

## Done when

- Each read listed above, over a reference that names no object or player (an illegal target, a
  departed operand not entitled to last known information, an unbound register), is unavailable
  through the channel `engine-statof-reference-channel` builds. The part of the effect that needs
  it doesn't happen [CR#608.2b]. An absent counter kind on a live object still reads 0.
- `Count::Damage` no longer panics on a reference that names no object.
- Axis of Mortality, one of whose targets gains hexproof before it resolves (a fixture derived
  from Leyline of Sanctity, "You have hexproof"), changes neither life total.
- Every other read in the list has a test grounded in a real card or a minimal repro derived from
  one.

## Not in scope

- `Count::StatOf`: `engine-statof-reference-channel`.
- An exchange that can't be completed for another reason (a player who can't gain or can't lose
  life [CR#701.12c]).

## Pointers

- `crates/engine/src/resolve/count.rs` (the arms above, `floated_mana`)
- `crates/engine/src/resolve/query.rs` (`eval_player_ref`, `eval_reference_product`)
- `v1/plugins/canon/cards/Axis of Mortality.ron`
