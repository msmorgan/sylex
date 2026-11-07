---
needs: []
---
# Delete the dead `OrderReplacements` decision

## Why

When several replacement or prevention effects apply to one event, the affected player chooses
one, it applies, and the process repeats over what still applies [CR#616.1,616.1f]. The engine
already does this through `ChooseReplacement` and `replace_registry::surface_choice`.
`OrderReplacements` in `decide/pending/choice.rs` is a second, never-constructed decision kind
for the same choice: its `resolve` is a `todo!` and `strategy.rs` lists it among the kinds with no
fallback. It is dead code that suggests unbuilt work.

## Done when

- `OrderReplacements`, its `DecisionPointKind` variant, its re-exports and its `strategy.rs` arm
  are gone.

## Not in scope

- The [CR#616.1a..616.1d] tiers of the replacement choice: `engine-replacement-batch-suspend`.

## Pointers

- `crates/engine/src/decide/pending/choice.rs`, `crates/engine/src/decide/mod.rs`,
  `crates/engine/src/decide/pending/mod.rs`, `crates/engine/src/lib.rs`
- `crates/engine/src/strategy.rs`
- `crates/engine/src/replace_registry.rs` (`surface_choice`)
- `docs/tickets/planned/engine-shell-decision-strategies.md` names this ticket
