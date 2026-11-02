---
needs: [ci-lean-gate, lean-card-soundness-gate, idris-retire-cards-idr, entailments-table-without-idris, docs-lean-is-the-workbench, tickets-idris-sweep]
---
**Retire the Idris workbench.** Updated 2026-11-02 (owner): Idris is deleted before Lean, which
`semantics-rust-rewrite` retires, so nothing here may hand work to Lean. The Idris card gate
(`idris_emit.rs`, `cargo xtask idris-check`, the per-plugin baselines, the CI `idris` job) goes in
this landing too, without a successor: it checks the v1 plugins, which are deletion-bound, and
`semantics-v3-parity` is the v3 cross-check. This moves the gate's retirement forward from
`semantics-v1-cutover` (ruling of 2026-09-06). Decision (2026-09-05): the Lean 4 workbench
under `lean/` is the successor of the Idris 2 workbench under `idris/`; the
Idris tree is a reference only, reread when a Lean port needs its evidence,
and is deleted once nothing else depends on it. This ticket is the
coordinator; its `needs` are the dependencies, in the order they unblock:

1. `ci-lean-gate`: Lean is built in CI at all.
2. `lean-card-soundness-gate`: the card-data gate (`idris-check`,
   `idris_emit.rs`, per-plugin baselines) has a Lean successor.
3. `idris-retire-cards-idr`: the 71-card coverage inventory in `Cards.idr` is
   migrated into `semantics-v3` (`lean/Semantics/Cards/` and `plugins_v3/`).
4. `entailments-table-without-idris`: the engine's entailment table is emitted
   without `EmitTables.idr`.
5. `docs-lean-is-the-workbench`: prose names the right workbench; the ADR is
   recorded.
6. `tickets-idris-sweep`: live tickets no longer gate on or deliver into Idris.

Then, in this ticket's own landing: delete `idris/` (48 `.idr` files, 47,518
lines, two `.ipkg`, six scripts, `VERIFY.md`), the CI `idris` job, `cargo xtask
idris-check`, `cargo xtask facts generate/check/labels` in their Idris form
(`lean-facts-tables-from-registries` re-homes the generator; if it lands
first, only the `.idr` output path goes), `cargo xtask map idris` and
`map idris-dead` (add `map lean` for the Lean constructors first, or fold it
into the same landing), the `idr` entry in `cite-config.json` and
`crates/xtask/src/coverage.rs`, and `docs/archive/idris-workbench-closure-tables.md`.
The `rs`-side doc comments that cite `idris/src/Semantics.idr` constructors
(`deckmaste_core` and `deckmaste_semantics` `action.rs`, `continuous.rs`,
`stat_value.rs`; `deckmaste_engine` `activate.rs`, `trigger.rs`) are
re-pointed at the `.syl` definitions they correspond to, or dropped; `core-idris-prose-sweep` covers the
larger `deckmaste_core` set and should land before or with this.

Landing record must list every CR citation that lived only in `.idr` files
(the coverage ratchet will otherwise drop them silently) and where each now
lives outside Lean (`.syl`, Rust or docs), since the Lean checker is retired next.

## Citations carried out of `idris/`

The 25 rules whose only Bound citation lived in `.idr` files, and where each now lives:

| Rule | Now cited in |
| --- | --- |
| [CR#107] | `plugins_v3/builtin/primitives/quantity/numberOf.syl` |
| [CR#107.2] | `plugins_v3/builtin/primitives/quantity/amountAggregate.syl`; `crates/deckmaste_core/src/count.rs` (`AggregateOp`) |
| [CR#118] | `plugins_v3/builtin/primitives/cost/compound.syl` |
| [CR#118.7a] | `plugins_v3/builtin/primitives/cost/generic.syl` |
| [CR#204.2] | `crates/deckmaste_core/src/token.rs` (`Token::color_indicator`) |
| [CR#205.1] | `plugins_v3/builtin/primitives/static/typeLine.syl` |
| [CR#205.3c] | `crates/deckmaste_core/src/type.rs` (`Subtype::types`) |
| [CR#205.3d] | `plugins_v3/builtin/primitives/static/typeLine.syl` |
| [CR#208.3] | `plugins_v3/builtin/primitives/quantity/statOf.syl` |
| [CR#404.1] | `crates/deckmaste_engine/src/resolve/query.rs` (`TopOfGraveyard`) |
| [CR#505.1a] | `crates/deckmaste_core/src/event.rs` (`PhaseKind::PrecombatMain`) |
| [CR#506.8] | `plugins_v3/builtin/primitives/timing/relativeToEvent.syl` |
| [CR#508.1c] | `plugins_v3/builtin/primitives/static/staticSpecCant.syl` |
| [CR#603.12a] | `plugins_v3/builtin/primitives/action/triggerReflexively.syl` |
| [CR#614.13] | `plugins_v3/builtin/primitives/static/before.syl` |
| [CR#616] | `plugins_v3/builtin/primitives/static/replace.syl` |
| [CR#700.8b] | `plugins_v3/builtin/templates/amount/partySizeOf.syl` |
| [CR#700.12a] | `plugins_v3/builtin/templates/predicate/isOutlaw.syl` |
| [CR#702.139b] | `plugins_v3/builtin/definitions/keyword_ability/companion.syl` |
| [CR#706.7] | `plugins_v3/builtin/primitives/action/rollDice.syl` |
| [CR#709] | `crates/deckmaste_card/src/card.rs` (`Card::Split`) |
| [CR#710] | `crates/deckmaste_card/src/card.rs` (`Card::Flip`) |
| [CR#715] | `crates/deckmaste_card/src/card.rs` (`Card::Adventurer`) |
| [CR#723] | `crates/deckmaste_engine/src/decide/mod.rs` (`Action::Concede`) |
| [CR#730] | `plugins_v3/builtin/primitives/action/merge.syl` |
