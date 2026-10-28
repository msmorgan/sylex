---
needs: []
---
# Parallelize and slim the Lean build

Standard constraints apply. `cargo xtask lean check` takes about 15 minutes wall-clock, nearly
all of it single-threaded. The measurements below are from 2026-10-28, on a 24-core machine.

| Target | Wall time | Bottleneck |
|---|---:|---|
| `Semantics` (33 modules) | ~2.0 min | A serial chain: `Abilities.lean` (~41 s, 48.4 MB `.olean`), `Definitions.lean` (~40 s, one `load_bundle` over 2,073 builtin files), `Check/Phrase.lean` (~25 s) |
| `SemanticsCards` | ~4.2 min | One module loads all 908 canon cards in sequence. About 223 s of it is kernel `decide` on `Card.check card = []` inside `elabSpelled`, ~27 s is code generation, and peak memory is 2.95 GB. |
| `SemanticsProofs` (80 modules) | ~8.8 min | 32 modules import `Semantics.Cards` and so wait for it, and the `Proofs/Cards*.lean` modules form an import chain 7 deep |

## Decisions

- **Keep the kernel proofs.** Each card's `ok` proof stays kernel-checked `decide`; no
  `native_decide` and no `Lean.ofReduceBool`. Keep the fast `evalExpr` pass, which gives the
  readable refusal messages.
- **Leave `Definitions.lean` alone.** It still loads the builtin in one module; that is next on
  the critical path, for a later ticket.

## Changes

1. **Drop unused recursors in `Abilities.lean`.** Remove `mkRecOn` and `mkBelow` from
   `runFastInductives` (`lean/Semantics/Authoring.lean`).
   - Recursion over the Abilities spine uses `fuel`, and `casesOn` is built from `rec`, so
     neither `*.recOn` nor `*.below` is referenced anywhere. Together they are 54.8% of the
     `Expr` objects in `Abilities.olean`.
   - Record in `lean/CONTRACTS.md` that recursion over the spine must use fuel. Structural
     recursion over it then fails loudly instead of puzzling the next author.
2. **Give `Semantics.Definitions` its own `lean_lib`.**
   - `needs = ["builtinBundle"]` sits on the whole `Semantics` library. Lake hashes that input
     into every module of the library, so any builtin edit rebuilds `Abilities.lean` (~41 s) and
     `Check/Phrase.lean` (~25 s), though only `Definitions.lean` reads the bundle.
   - Move `Semantics.Definitions` into a library of its own carrying the `needs`, and drop it
     from `Semantics`.
3. **Shard `Semantics.Cards` into deterministic modules.**
   - Assign each card to a shard by a stable hash of its filename (16 shards for now). Adding a
     card must not move any other card.
   - Each shard tracks only its own files as build inputs. The current single `canonBundle`
     `input_dir` covers all of `plugins_v3/canon`, which would rebuild every shard on any edit.
     - In Lake 5.0, `needs` is per library, and TOML `input_dir` filters support only
       `startsWith`, `endsWith` and `mem`. So each shard is its own `lean_lib` with its own
       `input_dir`.
     - Assign files to shards either in `lakefile.lean`, with a filter that computes the shard,
       or through generated `mem` lists in `lakefile.toml`, plus a check that fails when a card
       file is in no list.
   - `Semantics.Cards` becomes an umbrella that imports the shards, and `cardNamed` resolves a
     card in any shard.
4. **Stop generating code for card constants.**
   - Add `noncomputable` to the modifiers when `kind == card` (`Authoring.lean` around `:1584`).
   - `elabSpelled` runs `evalExpr` on the `Card` term before the `Spelled` `def` is added, and
     `Spelled.ok` is a `Prop` field checked by the kernel, so neither needs compiled code for the
     constant. Keep it that way.
   - Test: a failing card still reports its refusals.
5. **Decouple the proofs from `Semantics.Cards`.**
   - Delete the three dead imports (`CardsCopy`, `CardsMana`, `CardsPiles`).
   - Move the shared bench helpers into `Proofs/CardsHelpers.lean`, breaking the 7-deep chain
     and the imports of `Cards*` modules from `Anaphora`, `Damage`, `Deontic`, `Keyword`,
     `KeywordBodies` and `UntapStep`. As of 2026-10-28 the helpers are:
     - `CardsDescription`: `targetPlayerOrPlaneswalker`, `targetOpponentOrPlaneswalker`,
       `bioplasmAfterExile`;
     - `CardsAnaphora`: `eachCreatureThatSplitControls`, `opponentWithMoreLands`,
       `bioplasmAfterTest`;
     - `CardsTrigger`: `tahngarthHeader`;
     - `CardsChoice`: `tahngarthChoosesDefender`;
     - `CardsKeyword`: `thickSkinnedGoblinEchoPayment`, `protectionAbilities`, `wardAbilities`,
       `hexproofAbilities`;
     - `CardsTurn`: `ashnodsBattleGear` and `okAshnodsBattleGear`, used by `UntapStep`;
     - `CardsDeontic`: `okCloudspireCaptainCrewLine`, used by `Deontic`.

     A helper that is itself a card sub-term comes from the loaded card, as below, not from a
     copy.
   - `CardsTrigger.lean` has three theorems that restate a canon card's own `ok` proof:
     `(cardNamed "Frost Walker").card.check = []`, and the same for "Segmented Wurm" and
     "Angelic Cub". Retire them, naming each one: the card's `Spelled.ok` already proves exactly
     this when the card loads.
   - Proofs that need canon cards import only the shard modules that hold those cards.
     - Do not load card files from inside a proof module (`load_cards`, `IO.FS.readFile`). Lake
       does not track files an elaborator reads, so the proof would not rebuild when its card
       changed. Each such module would also re-prove its cards' kernel `decide` (about 65 s
       across the 123 card uses).
     - Importing a shard keeps both the dependency and the proof single. If a card moves to
       another shard, the proof fails to find it with `cardNamed` and must update its import:
       loudly, never silently.
     - Never copy card terms into proof files: a copy would let a proof pass against a stale
       card. That applies to the 61 `(cardNamed …).card` sub-term extractions too, which keep
       reading the imported cards.

## Acceptance

- `cargo xtask lean check` passes with no warnings, and no theorem depends on
  `Lean.ofReduceBool`.
- No module in `lean/Semantics/Proofs/` imports the `Semantics.Cards` umbrella, and none reads
  card files directly.
- No `recOn`, `below`, `brecOn` or `termination_by` over the spine types; checked by grep.
- Touching one canon card rebuilds exactly one shard and only the proof modules that import
  that shard.
- Touching one builtin file does not rebuild `Abilities.lean` or `Check/*`.
- Per-module build times before and after are recorded in the change description. The target is
  about 3 minutes wall-clock for a clean `lean check`.
