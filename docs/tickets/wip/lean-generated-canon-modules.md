---
needs: [lean-module-layering]
---
# Generate canon card modules inside Lake, and split the build into targets

Standard constraints apply. This is the second half of the Lean build speedup, after
`lean-module-layering`.

## The defect

`Semantics.Cards` runs `load_bundle "../plugins_v3/canon"`. That reads all 908 card files with
`IO.FS.readFile` inside one elaborator, on one core.

- Lake cannot see files an elaborator reads. Tracking happens only through the library-wide
  `canonBundle` input, so any card edit rebuilds the whole module (~4.2 min).
- About 223 s of that is kernel `decide` on `Card.check card = []`, run inside `elabSpelled`
  together with elaboration. So checker edits re-elaborate every card.
- 32 proof modules import the whole module.
- `defaultTargets` builds everything, proofs included, on every `lake build`.

## Rulings (owner, 2026-10-28)

- **Generation is part of Lake.** A bare `lake build` and the Lean language server
  (`lake setup-file`) must pick up plugin edits on their own, with no `xtask` step and no
  watcher. This replaces `lean-lake-native-generation`, which is deleted.
- **Card proofs stay kernel `decide`.** No `native_decide` and no `Lean.ofReduceBool`. The canon
  is expected to shrink, but Lean will later check card definitions produced by the CxG layer,
  heading toward the 32k corpus. Keeping data separate from checks leaves room for a compiled
  verifier on the Check side (`lean-compiled-card-verifier`, maybe).
- **Proofs are not built by default.** The default `lake build` is the fast loop.

## Lake mechanics

Verified on Lean 4.35 / Lake 5.0, by reading the Lake source under
`~/.elan/toolchains/leanprover--lean4---v4.35.0/src/lean/lake/` and by throwaway experiments
(Antigravity consult, 2026-10-28). Paths below are relative to that directory.

- **`extraDepTargets` alone is unsafe.**
  - When a library is built directly, `Module.recFetchInput` (`Lake/Build/Module.lean:30-38`)
    reads and hashes each module's `.lean` source before `recFetchPreSetup` awaits the library's
    `extraDep` (`:557-571`).
  - So a generator run as an extra dependency rewrites a shard after Lake has already hashed the
    old text. The stale `.olean` is kept. On a clean tree, the build fails on the missing file.
- **Override `Module.inputFacet` in `lakefile.lean`.** Make it await `mod.lib.extraDep.fetch`
  before reading `mod.leanFile`. Every path through a module then sees freshly generated source.
  - Lake folds package facet declarations over its built-in facets
    (`Lake/Load/Workspace.lean:39`), so a `module_facet` with that name replaces the built-in one.
  - The paths covered include `lake build`, `lake setup-file`, `lake lean` and transitive imports.
- **The generator target calls `setTrace .nil`.** `recFetchPreSetup` mixes the library's
  `extraDep` trace into every module's trace (`Lake/Build/Module.lean:603`). With a non-empty
  trace, any card edit would rebuild every shard. With `.nil`, each shard rebuilds only when its
  own generated text changes.
- **The language server picks edits up.** `lake setup-file` runs the generator: through
  `setupEditedModule` (`Lake/Build/Module.lean:1433-1442`) for the edited file's library, and
  through import fetching for every imported library. `lake lean` runs it too. `lake env` does
  not, and nothing depends on that.

## Changes

1. **Convert `lean/lakefile.toml` to `lean/lakefile.lean`,** keeping its options and libraries.
2. **`target genCanon`** is written in Lean inside `lakefile.lean`, with no Cargo in the loop. It
   reads `plugins_v3/canon/cards/` and writes gitignored modules, rewriting only files whose text
   changed. It ends with `setTrace .nil`.
   - **Fixed module names only.** Lake must know the module set before it builds, so a card edit
     never changes it.
   - **`Semantics/Cards/Data/Fixtures.lean`** holds every card that a proof references. That set
     is found from `cardNamed "…"` uses under `lean/Semantics/Proofs/`, or from a committed
     manifest the generator checks against them; choose one.
   - **`Semantics/Cards/Data/Shard00..15.lean`** hold the rest. Assign each card by a stable hash of
     its filename, so adding a card moves no other card. Cards in `Fixtures` are excluded, so each
     card is elaborated exactly once.
   - Data modules define `noncomputable` `Card` constants, import `Semantics.Definitions`, and
     never import `Semantics.Check`.
   - **`Semantics/Cards/Check/Fixtures.lean`** and **`Check/Shard00..15.lean`** each import their
     data module and `Semantics.Check.Card`. For each card they run the `evalExpr` refusal report
     and prove `theorem ok_<card> : <card>.check = [] := by decide`.
   - Above each generated card, write `-- source: plugins_v3/canon/cards/<file>.lean:<line>`. The
     source text follows verbatim, so an error at a generated line reads straight back to its
     origin.
3. **Override `Module.inputFacet`** as described above, in about 15 lines. Comment it with the
   Lake source references, because it depends on Lake's internal facet wiring.
4. **Delete** the canon `load_bundle` use, any `load_cards`, and the `canonBundle` `input_dir`.
5. **Targets:**

   | Library | Contents | Default |
   |---|---|---|
   | `Semantics` | AST, checker, elaborator | yes |
   | `SemanticsDefinitions` | `Semantics.Definitions` (builtin) | yes |
   | `SemanticsCheckBuiltin` | `Semantics.Check.Builtin` (builtin `ok_` theorems) | yes |
   | `SemanticsCardsData` | `Cards.Data.Fixtures`, `Cards.Data.Shard00..15`; needs `genCanon` | no |
   | `SemanticsCards` | `Semantics.Cards` umbrella over `Cards.Check.*`; needs `genCanon` | no |
   | `SemanticsProofs` | `Semantics.Proofs`; imports `Cards.Data.Fixtures`, never `SemanticsCards` | no |

   - **Inner loops:** `lake build Semantics` for checker work, `lake build` for builtin work,
     `lake build SemanticsCards` for card work, and the editor or `lake build SemanticsProofs` for
     proofs.
   - **The gate:** `cargo xtask lean check` builds every library with `--wfail`.
   - Update the usage comment at the top of the lakefile and `lean/README.md`.
6. **Proofs** import `Semantics.Cards.Data.Fixtures` for canon cards, never the umbrella or a check
   shard.
7. **Retire or rewrite** the proof tests that exercise canon `load_bundle` itself (in
   `Proofs/Authoring.lean`), naming each one. The builtin `load_bundle` stays until
   `plugins-v3-data-language` generates the builtin through the same mechanism.

## Regression test

The facet override depends on Lake internals, and a toolchain bump could bring stale builds back
silently. Add a test that rebuilds Gemini's experiment against the real lakefile: a throwaway
copy, or a tiny fixture package that shares the override code. It must show:

- a clean build with no generated files present succeeds;
- editing one card rebuilds exactly that card's data shard and check shard, and nothing else;
- editing a fixture card rebuilds `Fixtures` and the proofs that import it;
- `lake setup-file` on a proof module regenerates after a card edit.

Run it from `cargo xtask lean check`, or as an `xtask` test.

## Acceptance

- After editing a card, a bare `lake build SemanticsCards` and the language server need no other
  step.
- No module in `lean/Semantics/Proofs/` imports `Semantics.Cards` or a check shard.
- No Lean code reads `plugins_v3/canon` at elaboration time; only the Lake generator does.
- Editing a `Check/*` module re-elaborates no card data.
- A deliberately failing card reports its refusals next to its `-- source:` line.
- The regression test passes.
- Per-module build times before and after are recorded. The target is about 2 minutes or less for
  a clean `lean check`.
