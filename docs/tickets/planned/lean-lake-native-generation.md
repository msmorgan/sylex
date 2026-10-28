---
needs: [lean-generated-canon-modules]
---
# Make plugin-module generation part of Lake

Standard constraints apply. `lean-generated-canon-modules` generates Lean modules from
`plugins_v3/` with `cargo xtask lean gen` before `lake` runs. That works for `xtask lean check`
and freshly provisioned workspaces. It does not work for a bare `lake build`, or for the
lean-lsp editor server after a plugin edit: both see stale modules until someone runs gen. The
staleness check makes that loud, but not automatic.

Make generation a Lake step, so that `lake build` and the language server are always correct
on their own.

- Lake computes the import graph before it builds anything. The design must therefore produce the
  module set before Lake reads imports.
- Candidates:
  - a `lakefile.lean` that generates the modules when the package is configured;
  - Lake targets that produce the modules, with an umbrella whose imports are fixed;
  - a Lake script that the editor integration invokes.

  Measure each against Lake 5.0 / Lean 4.35, and pick the one that leaves every module traced
  by its source hash.
- The same mechanism must serve the builtin modules once `plugins-v3-data-language` generates
  them.
- Once it works, delete `cargo xtask lean gen`, or keep it only as a thin wrapper.

## Acceptance

- After editing a plugin file, a bare `lake build` (and lean-lsp) rebuilds exactly the affected
  modules without any `xtask` step.
- No module goes untracked.
