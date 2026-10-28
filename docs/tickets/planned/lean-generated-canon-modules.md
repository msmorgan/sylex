---
needs: [lean-module-layering]
---
# Generate real Lean modules for canon cards, splitting card data from card checks

Standard constraints apply. This is the second half of the Lean build speedup, after
`lean-module-layering`.

Today `Semantics.Cards` runs `load_bundle "../plugins_v3/canon"`, which reads all 908 card files
with `IO.FS.readFile` inside one elaborator, on one core.

- Lake cannot see files an elaborator reads, so tracking happens only through the library-wide
  `canonBundle` input. Any card edit rebuilds the whole module (~4.2 min).
- About 223 s of that is kernel `decide` on `Card.check card = []`, run inside `elabSpelled`
  together with elaboration. So checker edits re-elaborate every card.
- 32 proof modules import the whole module.

## Rulings (owner, 2026-10-28)

- **Generation is run by `xtask`.** `cargo xtask lean gen` writes gitignored `.lean` modules
  before `lake` runs. `cargo xtask lean check` runs gen first, and `scripts/provision-workspace`
  runs it so fresh workspaces work with lean-lsp at once. Making generation Lake-native is a
  follow-up: `lean-lake-native-generation`.
- **Card proofs stay kernel `decide`.** No `native_decide` and no `Lean.ofReduceBool`. The canon
  is expected to shrink, but Lean will later check card definitions produced by the CxG layer,
  heading toward the 32k corpus. The Data/Check split below keeps that open: a compiled verifier
  could replace or augment the Check side alone (`lean-compiled-card-verifier`, maybe).

## Changes

- **Generator.** `cargo xtask lean gen` reads `plugins_v3/canon/cards/` and writes gitignored
  modules. It rewrites only files whose content changed, so Lake's hashes stay stable.
  - **`Semantics/Cards/Data/<Card>.lean`**, only for cards a proof uses. Find those from
    `cardNamed "…"` uses under `lean/Semantics/Proofs/`. Each defines the card's `noncomputable`
    `Card` constant. It imports `Semantics.Definitions` and never `Semantics.Check`.
    - Module names are a deterministic, identifier-safe mangling of the printed name, because
      Lean module names cannot hold spaces or `//`.
    - The generator writes the name-to-module map next to the modules.
  - **`Semantics/Cards/Data/ShardNN.lean`** (16 shards, by a stable hash of the filename, so
    adding a card moves no other). Each defines its remaining cards, and imports, never
    restates, the per-card modules of its cards. Every card is elaborated exactly once.
  - **`Semantics/Cards/Check/ShardNN.lean`** imports its data shard and `Semantics.Check.Card`.
    For each card it runs the `evalExpr` refusal report (readable failures) and proves
    `theorem ok_<card> : <card>.check = [] := by decide`.
  - **`Semantics/Cards.lean`** becomes an umbrella that imports the check shards. `cardNamed`
    resolves through the data modules.
  - **Source positions.** Each generated module copies the source verbatim under a header naming
    its original path and line offset. `xtask lean` maps reported positions, parser errors
    included, back to the `plugins_v3/` file and line.
  - **Staleness.** Each generated module records the hash of its sources. A bare `lake build` run
    on stale modules fails with a message to run `cargo xtask lean gen`.
- **Delete** the canon `load_bundle` use, any `load_cards`, and the `canonBundle` `input_dir`.
- **Proofs** that inspect canon cards import `Semantics.Cards.Data.<Card>` and never the
  umbrella or the shards. They do not wait on any card proofs, and they never re-elaborate or
  re-prove a card.
- **Retire or rewrite** the proof tests that exercise canon `load_bundle` itself (in
  `Proofs/Authoring.lean`), naming each one. The builtin `load_bundle` remains until
  `plugins-v3-data-language` generates the builtin as modules too.

## Acceptance

- No module in `lean/Semantics/Proofs/` imports `Semantics.Cards` or a shard.
- No Lean code reads `plugins_v3/canon` at elaboration time.
- Editing one card regenerates exactly one source module. Lake then rebuilds only that card's
  data module or shard, its check shard, and the proof modules that import that card.
- Editing a `Check/*` module re-elaborates no card data.
- A deliberately failing card reports its refusals at its original file and line.
- Bare `lake build` on stale generated modules fails with the gen instruction.
- Per-module build times before and after are recorded. The target is about 2 minutes or less
  for a clean `lean check`.
- Gate: `cargo xtask lean check`. Add an `xtask` test for the generator: stable output, only
  changed files rewritten, position mapping, and the staleness check.
