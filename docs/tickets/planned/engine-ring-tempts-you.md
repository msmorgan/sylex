---
needs: [core-emblems, core-getdesignation-scopes-and-eviction]
---

# The Ring tempts you: the Ring emblem and Ring-bearer in the engine

## Why

54 Vintage-legal cards say "the Ring tempts you". `.syl` writes it
(`keyword_action/theRingTemptsYou.syl`, `designation/ringBearer.syl`), but core and the engine
cannot run it. Emblems exist (`GetEmblem` puts one in the command zone), and core's designation
model can describe a Ring-bearer (object scope, `DesignationUniqueness::PerPlayer`), but three
seams are missing.

## Done when

- Granting Ring-bearer to a creature ends it on that player's previous Ring-bearer, and it ends
  when another player gains control of the bearer [CR#701.54a]; it is not a copiable value
  [CR#701.54b]. The grant and eviction come from `core-getdesignation-scopes-and-eviction`.
- The engine counts how many times the Ring has tempted each player, and the emblem's three
  higher abilities are each in force as long as that count reaches two, three or four
  [CR#701.54c]; the emblem itself is not changed.
- The Ring tempts a player whenever they complete the [CR#701.54a] actions, even if some or all
  of them were impossible [CR#701.54d], so a "whenever the Ring tempts you" trigger fires with no
  creature to choose.
- Engine tests with real tempting cards cover the first temptation (emblem created, bearer
  chosen), a second bearer replacing the first, each tier's ability appearing at its count, and a
  temptation with no creature.

## Pointers

`crates/core/src/designation.rs`, `crates/core/src/action.rs` (`GetEmblem`,
`GetDesignation`), `plugins/builtin/definitions/keyword_action/theRingTemptsYou.syl`,
`plugins/builtin/definitions/designation/ringBearer.syl`.
