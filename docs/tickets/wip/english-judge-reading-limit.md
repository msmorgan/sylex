---
needs: [english-forest-counts-and-forks]
---

# Judgments checked on units of any size

## Why

The judge once refused any unit above a fixed Reading count, and the census searched only the
best 4096 Readings of a unit for an endorsed one, so a unit above the count could not carry a
judgment and its census verdict depended on rank, which `docs/decisions/english.md` §5 forbids.

## Design

- `READING_LIMIT` (4096, `crates/english/src/judgment.rs`) is deleted. The judge enumerates every
  Reading of a unit, and the census (`census::judge_parse`) considers every Reading when it
  looks for an endorsed one, so no verdict depends on rank.
- The largest unit in the pool has 9,216 Derivations (*Remove Enchantments*), and every unit
  enumerates (measured on the whole pool).

## Done when

- `READING_LIMIT` no longer exists, and no code fails, truncates or stops at a Reading count.
- A judgment on *Remove Enchantments* (9,216 Derivations) and one on *Maralen, Fae Ascendant*
  (5,955), each asserting its endorsed analysis, hold.
- `cargo xtask english strata --check` passes, and the landing reports any unit whose verdict
  changed, with the reason.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_english` and `xtask`.

## Not in scope

- The judgment entry syntax.
- Ranking: where the endorsed Reading ranks stays reported, never gated (`english.md` §5).

## Pointers

- `crates/english/src/judgment.rs` (`Judge::evaluate_scoped`), `crates/english/src/census.rs`
  (module doc, `judge_parse`).
- `crates/english/judgments/README.md`, `docs/decisions/english.md` §5.
