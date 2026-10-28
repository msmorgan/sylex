---
needs: [plugins-v3-expansion-baseline-recursive-samples]
---
# Eliminate the qualified sites and dependency-scan workarounds left by builtin cleanup

Standard constraints apply. Kept qualifications and workarounds are defects to eliminate, not
permanent exceptions.

## Sites

1. **`Semantics.` qualifications.** Change `ouqkwnvy` lists the 102 sites kept in step 2 of
   `plugins-v3-builtin-cleanup`. Reconcile that list with the current declarations; some may
   since have been shortened by `plugins-v3-canon-authoring-surface`.
2. **Partly unqualified sites that keep `Templates.<Sort>.`.** These pass the "zero
   `Semantics.`" check in letter only. In each, a parameter of the same name shadows the bare
   word:
   - `templates/amount/eventSum.lean` and `eventCount.lean` (`Templates.LookbackSpan.within`)
   - `templates/action/lookAt.lean` and `definitions/keyword_action/reveal.lean`
     (`Templates.Exposed.cards`)
   - `definitions/keyword_action/create.lean` (`Templates.CreationSpec.token`)
3. **Dependency-scan workarounds:**
   - `templates/ability/devourEntry.lean` writes `Action.devour` "so the untyped DAG does not add
     a false cycle edge".
   - `templates/zone/exileZone.lean` writes `zone .exile none` so the scan "does not form a
     cycle with the `exile` action".
   - The `exileZone` workaround is written into `lean/CONTRACTS.md` (around `:145`) and
     `lean/README.md` (around `:250`) as if it were the contract. Remove it from both once
     fixed.

## Causes to fix first

- **The six `Semantics.Definitions.*` calls that fail when unqualified** (`madness`, `airbend`,
  `devour`, `regenerate`, `regeneration`, `controllerSacrifices`).
  - **The root cause is not known.** The overnight log's hypothesis is that `referencedIdents`
    reads `structInstField` at `stx[1]` instead of `stx[2]`. That is wrong: in Lean v4.35,
    `stx[1]` holds the `:= value`.
  - The "untyped bundle-DAG cycle false positive" that change `ouqkwnvy` and the overnight log
    give as the cause is a guess. Each referenced action has a single declaration, so a bare
    reference produces a normal edge.
  - Reproduce each failure on a scratch copy and record the actual error before changing
    `bundleDependencies` or the elaboration order.
- **The untyped dependency scan.** `bundleDependencies` (Lean) and `Bundle::load` (Rust) add edges
  by bare name without regard to expected type. That is behind items 3's two workarounds. Make
  the edge typed by the reference's expected sort where it is known. Otherwise drop edges that
  would only close a cycle through a different return type, and report a cycle that remains
  ambiguous.
- **The `.sets` multi-token argument check** in `crates/deckmaste_semantics_v3/src/lower/compute.rs`
  (around `:495`, `Semantics.Characteristics.valueEdits`), including the typed conditional in
  `copyCharacteristics`.
- **Parameter-shadowed bare words** (item 2). Let a NounPhrase or Duration parameter coexist with
  a same-named word of another sort, the way `elabClosedWord` already prefers a compatible local.
- **Qualified helper and projection dispatch** in Templates (`Characteristics.*`, `Cost.asPaid`,
  `Location.observedOnDeparture`, `QualityPayload.edits`) and qualified pattern heads.

## Acceptance

- Zero `Semantics.` and zero `Templates.<Sort>.` qualifications in
  `plugins_v3/builtin/{definitions,templates,rules}`.
- `devourEntry` writes `devour`, and `exileZone` writes its bare form.
- The `exileZone` workaround text is gone from CONTRACTS.md and README.md.
- Focused Lean and Rust regressions for typed dependency edges, parameter-shadowed words and
  helper argument parsing.
- Builtin and canon expansion baselines show 0 differences.
- Gates: `cargo xtask lean check`. Run `cargo test` and clippy for `deckmaste_semantics_v3` and
  `xtask`.
