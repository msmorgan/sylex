---
needs: []
---

# Compact `xtask english forestwalk` tree and spine output

## Why

On long multi-clause Corpus Units such as *Remove Enchantments*, `cargo xtask english forestwalk` grows larger as a forest is narrowed: `Settled spine:` re-prints every already-resolved subtree down to leaves on each step, every branching Constituent repeats its full surface span at each depth, `--limit >= 2` prints the settled spine again inside every `Reading N:`, and `--limit 0` neither folds a 1-Derivation spine nor permits `--assertions`. Copying an aliased option line after its count also includes ` = <alias answer>`, which fails without pointing out the ` = ` suffix.

## Design

- Expanded branching nodes in `forestwalk`'s folded trees (`Settled spine:` and `Reading N:`) print `Category [Production]` without repeating the node's `"surface"` span; unexpanded nodes (`Alternatives`, `Splits`, and folded settled subtrees) keep `"surface"`.
- When `Settled spine:` is not a single printed Reading (`Derivations > 1` or `--limit 0`), each settled `SettledNode` whose subtree contains no `Alternatives` or `Splits` folds to one `Category "surface"` line (the root included when `Derivations == 1` and `--limit 0`) instead of expanding its descendants.
- When `2 <= Derivations <= --limit`, each `Reading N:` prints only the Reading's folded subtree(s) at the stop(s) of `Settled spine:`, in spine order and carrying each stop's function label on the spine (or the full Reading when no root Constituent is settled).
- `--assertions` accepts `1 <= Derivations <= max(--limit, 1)`, so `--limit 0 --assertions` isolates a single remaining Reading without printing its full tree.
- A `--pick` or `--exclude` selector whose answer contains ` = ` from an aliased option line fails naming the single question's answer without the ` = ` suffix, and `--help` notes that an option's answer stops before ` = `.

## Done when

- `Settled spine:` omits `"surface"` on expanded branching nodes and folds closed `SettledNode` subtrees when `Derivations > 1` or `--limit 0`.
- `Reading N:` for `2 <= Derivations <= --limit` prints only the subtrees at the stops of `Settled spine:` (or the whole tree when `SettledSpine::Roots`).
- `--limit 0` folds a 1-Derivation root to `Category "surface"` and works with `--assertions` when `Derivations == 1`.
- `--pick` and `--exclude` reject a copied ` = ` alias tail with an error naming the target question's own answer, and `--help` and `crates/english/README.md` state the current behavior.
- Unit tests in `crates/xtask/src/english/forestwalk/` cover closed-subtree folding on `Settled spine:`, stop-only `Reading N:` output under `--limit`, `--limit 0` with and without `--assertions`, and ` = ` alias-tail selector errors.
- `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D warnings` pass.

## Pointers

- `crates/xtask/src/english/forestwalk.rs`
- `crates/xtask/src/english/forestwalk/selector.rs`
- `crates/xtask/src/english/forestwalk/tests.rs`
- `crates/syntax/src/settled_spine.rs`
- `crates/english/README.md`
