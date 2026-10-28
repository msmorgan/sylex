---
needs: []
---
# Layer the Lean modules so data, checker and DSL elaborator rebuild independently

Standard constraints apply. This is the first half of the Lean build speedup. Its sibling
`lean-generated-canon-modules` follows.

Measurements from 2026-10-28, on 24 cores: a clean `cargo xtask lean check` takes about
15 minutes. Today the modules form one vertical chain, so touching anything rebuilds almost
everything below it:

- `Words.lean` imports the whole DSL elaborator (`Authoring.lean`, ~1,800 lines) just for
  `fast_inductives` and four tag attributes. Any elaborator edit therefore rebuilds `Abilities.lean`
  (~41 s) and all of `Check/*` (~35 s).
- `needs = ["builtinBundle"]` sits on the whole `Semantics` library. Lake hashes that input into
  every module of the library, so any builtin edit also rebuilds `Abilities` and `Check`.
- `Definitions.lean` imports `Check.Phrase` and `Check.Rules`, only because the generated `ok_`
  theorems for builtin rules sit inline. Every checker edit, which all 30 `lean-v3-*` tickets
  make, therefore re-elaborates all 2,073 builtin files.
- `Abilities.lean` declares 70 inductive types in one `mutual` block, of which only ~15–20
  genuinely recurse into one another. `runFastInductives` also generates `recOn` and `below`,
  which nothing references; they are 54.8% of the `Expr` objects in `Abilities.olean`, which is
  48.4 MB.

## Changes

1. **Extract `Semantics/FastInductives.lean`** with `fast_inductives` and the core tag
   attributes. `Words.lean` imports it instead of `Authoring`. The AST (`Semantics`) and the
   checker (`Semantics.Check`) then import no elaborator code.
2. **Move `Spelled` out of `Check/Card.lean`** into a module the data side can import without the
   checker.
3. **Give `Semantics.Definitions` its own `lean_lib`,** carrying `needs = ["builtinBundle"]`.
   Remove the `needs` from `Semantics`.
4. **Keep `Definitions` free of `Semantics.Check`.** Emit the builtin `ok_` theorems
   (`Rule.check`, `RegistryDefinition.check`) into a separate `Semantics/Check/Builtin.lean`.
   `Definitions` and `Check` then build in parallel, and checker edits never re-elaborate the
   builtin.
5. **Slim `Abilities.lean`:**
   - Drop `mkRecOn` and `mkBelow` from `runFastInductives`. Record in `lean/CONTRACTS.md` that
     recursion over the spine uses fuel.
   - Move leaf types that do not refer to the spine above the `mutual` block, and wrappers no
     spine type refers to below it. Leave only the genuinely mutually recursive types inside.
     A type that a spine constructor contains stays in the block unless it is parameterized.
6. **Prepare the proofs for per-card imports:**
   - Delete the dead `import Semantics.Cards` in `CardsCopy`, `CardsMana` and `CardsPiles`.
   - Move the shared bench helpers into `Proofs/CardsHelpers.lean`. That breaks the 7-deep
     `Proofs/Cards*` chain, and the `Cards*` imports from `Anaphora`, `Damage`, `Deontic`,
     `Keyword`, `KeywordBodies` and `UntapStep`. As of 2026-10-28 the helpers are:
     - `CardsDescription`: `targetPlayerOrPlaneswalker`, `targetOpponentOrPlaneswalker`,
       `bioplasmAfterExile`;
     - `CardsAnaphora`: `eachCreatureThatSplitControls`, `opponentWithMoreLands`,
       `bioplasmAfterTest`;
     - `CardsTrigger`: `tahngarthHeader`;
     - `CardsChoice`: `tahngarthChoosesDefender`;
     - `CardsKeyword`: `thickSkinnedGoblinEchoPayment`, `protectionAbilities`, `wardAbilities`,
       `hexproofAbilities`;
     - `CardsTurn`: `ashnodsBattleGear`, `okAshnodsBattleGear`;
     - `CardsDeontic`: `okCloudspireCaptainCrewLine`.
   - A helper that is a sub-term of a canon card keeps reading the card. Never copy it.
   - Retire, by name, the three `CardsTrigger.lean` theorems that restate a card's own `ok`
     proof: `(cardNamed "Frost Walker").card.check = []`, and the same for "Segmented Wurm" and
     "Angelic Cub".

## Acceptance

- Editing `Authoring.lean` rebuilds no module of `Semantics` (the AST) or `Semantics.Check`.
- Editing a builtin plugin file rebuilds neither `Abilities.lean` nor `Check/*`.
- Editing a `Check/*` module rebuilds `Definitions` not at all.
- No `recOn`, `below`, `brecOn` or `termination_by` over the spine types; checked by grep.
- Kernel-checked `decide` throughout. No theorem depends on `Lean.ofReduceBool`.
- Per-module build times before and after are recorded in the change description.
- Gate: `cargo xtask lean check`.
