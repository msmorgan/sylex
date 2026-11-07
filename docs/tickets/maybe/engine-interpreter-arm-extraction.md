---
needs: [ci-cite-gate]
---
# Consider extracting the largest arms of `run_effect`

## Why

`run_effect` in `crates/engine/src/resolve/effect.rs` is one match over `Instruction`, about 1,100
lines long. Its largest arms (`Simultaneously`, `Continuously`, `Each`, `Modal`, then `Batch`, `Act`
and `May`) are natural semantic units whose contracts are hard to see inline. Extracting some of
them into named helpers may help local reasoning; whether it does, and which arms, is the owner's
call. Line counts are not the reason.

## Done when

- The owner has approved a list of arms to extract.
- Each approved arm body is a named helper in the same module, with control flow and returns
  unchanged and its citations and invariant comments carried along; `run_effect` stays one flat
  match with one arm per variant.
- Each extracted arm has a focused test, the engine's test count is unchanged otherwise, and the
  multiset of comments and citations in the file is unchanged.
- `cargo fmt --check` passes.

## Not in scope

- Traits, a second dispatch table, or moving arms into other modules.
- Moving the file's embedded tests (`docs/tickets/planned/split-resolve-tests.md`).

## Pointers

- `crates/engine/src/resolve/effect.rs` (`run_effect`)
- `crates/core/src/effect.rs` (`Instruction`)
