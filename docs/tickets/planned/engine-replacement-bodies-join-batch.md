---
needs: []
---

# Engine: the modified events of one batch's replacements stay in that batch

## Why

A replaced event never happens; a modified event occurs instead [CR#614.6], and one event can
contain several occurrences [CR#603.2c]. When a replacement applies to a member of an
`Occurrence::Batch`, `schedule_body` (`crates/engine/src/replace_registry.rs`) schedules its
`instead`/`also` body as its own `RunEffect`, which runs after the batch commits and emits its own
occurrence. So the modified events of one simultaneous event split into one occurrence per
member. Day of Judgment under Rest in Peace exiles each creature as a separate occurrence, and
Ketramose, the New Dawn ("Whenever one or more cards are put into exile from graveyards and/or
the battlefield during your turn, …") triggers once per creature instead of once. Two
regenerating Merfolk under Deeproot Pilgrimage ("Whenever one or more nontoken Merfolk you
control become tapped, …") likewise tap in two occurrences.

## Done when

- The modified events that replacements produce for the members of one batch commit as one
  occurrence together with the members that were not replaced.
- Tests: Day of Judgment over two creatures under a Rest-in-Peace replacement triggers a
  Ketramose-shaped "one or more cards are put into exile" watcher once; two creatures that each
  regenerate from one destroy-all trigger a Deeproot-Pilgrimage-shaped "one or more … become
  tapped" watcher once.

## Pointers

- `crates/engine/src/replace_registry.rs` (`apply_one`, `schedule_body`, `resume_replacements`)
- `crates/engine/src/step/mod.rs` (`commit_occurrence`, `evolving_batch`)
