---
needs: []
---
# A bound keyword with a period reads as a keyword instruction

## Why

`KeywordInstruction` makes a Sentence of a keyword line item and a period when the item carries
a Keyword Parameter (*Equip {2}.*, *Suspend X.*), and refuses a bare keyword (*Hexproof.*),
which takes no period. A Bound Keyword counts its stem as its parameter (*forestwalk*: stem
*forest*, bound *walk*), so *Forestwalk.* and *Landwalk.* read as keyword instructions. No card
prints a bound keyword with a period.

## Design

- Retire unconditioned `KeywordInstruction` (`Sentence -> KeywordLineItem '.'`): standalone keyword
  lines without an em-dash cost or trailing supplement take no period (`docs/oracle-style-guide.md`
  §3), so both `*Forestwalk.` and standalone `*Equip {2}.` are refused, eliminating surplus
  `KeywordInstruction` derivations on quoted granted keywords.
- Fix `DashCost` vs `DashCondition` in `crates/vocabulary/src/lib.rs` so set-off em-dash costs value
  `FinalParameter = SetOffCost` consistently (`SetOffKeywordItem`), extend
  `KeywordFollowedBySentences` for keyword items followed by sentences on one line (*Enchant
  creature. …*), and admit a terminal `.` on a `KeywordLineItem` only when it carries a trailing
  supplement (*Firebending X, where X is … .*).

## Done when

- `KeywordInstruction` is retired, and neither `*Forestwalk.` nor standalone `*Equip {2}.` reads as
  a `Sentence`.
- Every corpus unit previously read through `KeywordInstruction` keeps a valid Reading through
  `SetOffKeywordItem`, `KeywordFollowedBySentences` or supplemented `KeywordLineItem`, and the
  census shows no verdict loss on the supported corpus.
- `cargo xtask english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass
  for `sylex_vocabulary` and `sylex_english`.

## Pointers

- `crates/english/grammar/english.fug`: `KeywordInstruction`, `KeywordForm`.
- `crates/vocabulary/src/lib.rs`, where a parameter pattern values `KeywordForm`.
