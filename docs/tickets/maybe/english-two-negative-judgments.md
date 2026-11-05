---
needs: []
---

# Experiment: retire two negative judgments and the rules behind them

**Why.** Two negative judgments may refuse forms nothing needs refused:
- `floated_each_quantifier.txt`: `* Remove it all.` Its own comment says *all* after an accusative
  personal pronoun is part of a compound pronoun, which licenses "it all".
- `notation_coordination.txt`: `*NounPhrase two or one creatures`. A descending numeral order no
  card prints; refusing it buys nothing on real text (`docs/decisions/english.md` §3).

**Done when.**
- Each entry is removed, or rewritten if it turns out to pin something real, and the reason is given
  in the landing.
- Any feature, equation or Production that exists only to refuse it is found and removed, or kept
  with the census ablation that shows it pays for itself (`english.md` §6).
- `cargo xtask english strata --check` and `cargo test -p sylex_english` pass.

**Pointers.** `crates/english/judgments/floated_each_quantifier.txt`,
`crates/english/judgments/notation_coordination.txt`, `crates/english/grammar/english.ron`,
`crates/english/README.md` (working rules).
