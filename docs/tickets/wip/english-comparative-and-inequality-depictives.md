---
needs: []
---
# Exclude comparative and inequality adjectives from depictive adjuncts

## Why

`InterleavedDepictive`, `AdjectivalDepictive`, `DepictiveAdjunct`, and `DepictiveAdjective` in `crates/english/grammar/english.fug` exclude `Grade = Equative | Mixed` but still admit `Grade = Comparative` and `Grade = NonScalarInequality`. Depictive and resultative predicative adjuncts denote a state of the predicand during or resulting from the action (*enters tapped*, *attacks alone*, *choose odd or even*, *guessed wrong*), never a comparative or inequality relation (CGEL, Ch. 4, §5.3, pp. 261–263; Ch. 6, §2.1, p. 529). Admitting `Comparative` lets bare `adj:less` and `adj:greater` attach as `DepictiveAdjective` on 124 Corpus Units (*Pteramander*: `costs [{1}]_{Object} [less]_{DepictiveAdjective} to activate` beside `[{1} less]_{Object}`) and as `DepictiveAdjunct` on 737 Corpus Units (*with power 3 or [less]*), with 0 Corpus Units needing `Comparative` or `NonScalarInequality` in any of the four Productions.

## Design

- Exclude `Comparative` and `NonScalarInequality` alongside `Equative | Mixed` on `InterleavedDepictive`, `AdjectivalDepictive`, `DepictiveAdjunct`, and `DepictiveAdjective` in `crates/english/grammar/english.fug`, updating the gate comment on `DepictiveAdjective` with witness *Pteramander* (`docs/design/english.md` §6).
- Add Judgment assertions in `crates/english/judgments/comparative_depictives.txt` pinning `!AdjectivePhrase "less" as Adjunct of cost` on *Pteramander*.

## Done when

- `InterleavedDepictive`, `AdjectivalDepictive`, `DepictiveAdjunct`, and `DepictiveAdjective` reject `Grade = Equative | Comparative | NonScalarInequality | Mixed`.
- *Pteramander* has no `DepictiveAdjective` Reading for `"less"`.
- `cargo xtask english strata --check` against a pre-change baseline shows 0 lost faces, 0 verdict regressions, and 0 Corpus Units with increased Readings.
- `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug` (`InterleavedDepictive`, `AdjectivalDepictive`, `DepictiveAdjunct`, `DepictiveAdjective`)
- `crates/english/judgments/comparative_depictives.txt`
- `docs/design/english.md` (§6)
