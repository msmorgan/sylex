---
needs: [engine-lki-robustness]
---

# Engine: the trigger scan enumerates the abilities a watcher has, not its printed ones

## Why

`scan_event` (`crates/engine/src/trigger.rs`) takes each watcher's triggered abilities from
`derive::derived_abilities_of`, which always starts from `derive::abilities_of_source`: the
printed list, plus conferred and layer-6-granted additions. Nothing removes a printed trigger the
layered view no longer has, and a watcher that has just left the battlefield (`Watcher::Leaving`)
has no live battlefield object, so `abilities_of_source` reads its front face. The leaving
watcher's `LkiSnapshot` already holds its derived abilities, but the scan never reads them.

A trigger exists only while its source has the ability, and a leaves-the-battlefield trigger
looks back at "the existence of those abilities ... immediately prior to the event" [CR#603.10].
So today:

- Footlight Fiend under Turn to Frog ("target creature loses all abilities") still deals 1 damage
  when it dies.
- Moonlit Wake that has lost all abilities still gains life when a creature dies.
- Glistening Goremonger (the back face of Pyretic Prankster) dies and the scan reads Pyretic
  Prankster's abilities, so its "When this creature dies" never triggers.

## Done when

- A live watcher triggers only from abilities its layered view has [CR#613.1f]; a printed trigger
  removed in layer 6 does not trigger.
- A leaving watcher triggers from its snapshot's abilities, including a back face's and a
  layer-6-granted one, and from none it had lost [CR#603.10a].
- A trigger fired from an ability with no index-stable printed identity still carries its body by
  value, as conferred and back-face triggers do now.
- Tests: Footlight Fiend dies after losing all abilities and does not trigger; Moonlit Wake that
  lost all abilities does not trigger on a creature dying; Glistening Goremonger dies and its own
  dies trigger fires.

## Pointers

- `crates/engine/src/trigger.rs` (`scan_event`, `Watcher`)
- `crates/engine/src/derive.rs` (`derived_abilities_of`, `abilities_of_source`)
- `crates/engine/src/lki.rs` (`LkiSnapshot::characteristics`)
- `docs/tickets/planned/engine-lookback-occurrence-watchers.md`: co-leavers as watchers, the
  same look-back rule.
