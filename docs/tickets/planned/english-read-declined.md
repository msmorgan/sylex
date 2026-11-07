---
needs: []
---

# Read the declined units and delete the declined mechanism

**Why.** The parser admits all English on all Magic cards (`docs/decisions/english.md` §4).
Declining a unit was a shortcut around that. Three Reserved List units are declined: Illusionary
Mask, Raging River and Soul Echo. This ticket reads them and then removes the mechanism.

**Spike first.** Before building anything, run each declined unit against the existing machinery and
report what is missing. For each unit, give the estimated size and location of its support:
grammar data in `english.fug`, lexicon data, or engine code. Bespoke engine code outside
`english.fug` is to be avoided. The owner decides per unit, from the spike, before it is built.
For Raging River the spike answers:
- Does quotation handling (`chart::quotations`, `unwritten_period` and the quotation Productions
  for granted abilities) already cover the period absorbed inside the closing quote?
- What does a quoted word used as a mention need, both as a premodifier (*a "left" pile*) and as an
  Object (*choose "left" or "right."*)? Is that a grammar-data change or engine code?

**Risks.**
- Forest growth.
- How many new features the three constructions need.
- Raging River's quoted pile labels, which swallow punctuation.

**Done when.**
- Each declined unit reads.
- The landing reports a census comparison in the form of an ablation: the Readings and units gained,
  and how much the largest units grow. Growth is contained structurally, by a gate justified by
  ablation or a better structural analysis (`english.md` §6), never by policing a word. The owner
  judges whether the cost is acceptable.
- The declined mechanism is gone: `declined.txt`, `DECLINED`, `StratumRow::declined`, the census
  accounting for declined units, its tests, and the "Declining" working rule in
  `crates/english/README.md`.
- `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

**Pointers.**
- `crates/english/declined.txt`: its three entries name the constructions (partitive right-node
  raising, quoted pile labels, a *that*-clause with a fronted distributive over a mass quantity).
- `crates/english/src/lib.rs` (`DECLINED`), `crates/english/src/census.rs`,
  `crates/english/tests/census.rs`, `crates/xtask/src/english.rs`.
- `crates/english/README.md`, "Working rules".
- `crates/syntax/src/chart.rs` (`quotations`, `unwritten_period`).
