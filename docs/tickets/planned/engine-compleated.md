---
needs: [engine-planeswalkers]
---
# Compleated: a planeswalker cast with life paid for Phyrexian symbols enters with two fewer loyalty counters per symbol

## Why

Compleated [CR#702.150a] modifies the planeswalker's starting-loyalty replacement [CR#306.5b]
by how many Phyrexian mana symbols [CR#107.4f] its caster paid with life. The engine concretizes
Phyrexian picks at announcement (`PendingStackEntry::concretized`), but that record does not
survive to the permanent entering, so the replacement has nothing to read.

## Done when

- A cast records how many Phyrexian symbols in its cost were paid with life, and the record is
  readable while the resolving permanent spell enters.
- The compleated replacement reads it: the planeswalker enters with its starting loyalty minus
  two per such symbol, and with its full loyalty when none was paid with life or when it was not
  cast.
- Tests with Tamiyo, Compleated Sage (loyalty 5, cost {2}{G}{G/U/P}{U}): 3 loyalty counters when
  {G/U/P} is paid with life, 5 when paid with mana, 5 when put onto the battlefield without
  being cast. A core fixture is fine where the card is not authored.

## Pointers

- `plugins/builtin/definitions/keyword_ability/compleated.syl`: the declared replacement.
- `crates/engine/src/stack.rs`: `PendingStackEntry::concretized`.
- `crates/engine/src/cost_options.rs`: the Phyrexian readings.
- `crates/engine/src/replace.rs`: the conferred starting-loyalty replacement's fold into
  `EnterStatus`.
