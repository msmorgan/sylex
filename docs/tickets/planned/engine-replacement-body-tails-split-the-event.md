---
needs: []
---

# Engine: a replacement body's later parts split the event per member

## Why

A replaced event never happens; a modified event occurs instead [CR#614.6], and one event can
contain several occurrences [CR#603.2c]. `join_body` (`crates/engine/src/replace_registry/body.rs`)
joins a body's parts to the occurrence only up to the first part that does more than heal or tap.
Every later part runs after the commit, and each member's tail is its own `RunEffect` that emits
its own occurrence. Kalitas, Traitor of Ghet ("If a nontoken creature an opponent controls would
die, instead exile that card and create a 2/2 black Zombie creature token.") facing one destroy-all
over two creatures exiles both in the destroy-all's occurrence, but creates the two tokens in two
occurrences, so a "whenever one or more tokens you control enter" watcher triggers twice. The
Darkness Crystal ("instead exile it and you gain 2 life") splits its two life gains the same way.

## Open questions

- Which later parts can join the occurrence after all? Creating a token or gaining life reads
  nothing the earlier exile changed; a part that acts on the moved object
  (engine-replacement-body-finds-moved-object) does.
- Do the waiting tails of one occurrence's members commit together, as one occurrence after it?

## Done when

- The later parts of the bodies that replace the members of one occurrence commit as the rules
  require, each open question answered with evidence.
- Tests: Day of Judgment over two creatures an opponent controls under Kalitas, Traitor of Ghet
  triggers a "whenever one or more tokens you control enter" watcher once; the same under The
  Darkness Crystal gains its 4 life in the occurrences the answers above decide, and a "whenever you
  gain life" watcher triggers once per life gain event that gives [CR#119.9].

## Pointers

- `crates/engine/src/replace_registry/body.rs` (`join_body`, `simultaneous_part`)
- `crates/engine/src/step/mod.rs` (`commit_occurrence`)
- `crates/engine/tests/replace_registry.rs` (the Darkness Crystal test)
