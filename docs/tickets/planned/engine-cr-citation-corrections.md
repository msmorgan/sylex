---
needs: []
---
**Correct rule citations and rules claims in `crates/` comments where the behavior is already
right.**

## Why

Several comments and doc comments in `crates/core` and `crates/engine` cite the wrong rule, or
state the rule wrongly, around code that behaves correctly. Each one misleads the next reader. The
fixes change comments only, never behavior. Locate the sites by text; the files named below are
where each phrase occurs.

1. **Doubling Season has no "trigger half".** `TriggerMultiplier` is described as covering "the
   trigger half of Doubling Season", in `crates/core/src/continuous.rs` and twice in
   `crates/engine/src/trigger.rs`. Doubling Season is two replacement effects [CR#614.16]; a
   trigger multiplier is [CR#603.2d]. Use Elesh Norn, Mother of Machines' "triggers an additional
   time" clause as the example instead.
2. **Opponents are [CR#102.2,102.3].** [CR#102.1] defines player and active player, and
   [CR#102.4] defines "your team". Sites: `Count::Opponents` in `crates/core/src/count.rs`;
   `same_team` and `opponent_count` in `crates/engine/src/state.rs`; the `Count::Opponents` arms
   in `crates/engine/src/resolve/count.rs` and `crates/engine/src/layer.rs`; the whole-section
   [CR#810] cite on `TeammateOf` in `crates/core/src/filter.rs`. Opponent counts cite
   [CR#102.2,102.3], `same_team` cites [CR#102.3,102.4], `TeammateOf`'s "neither you nor an
   opponent" cites [CR#102.3], and [CR#810.1] stays as the team-game instance. Cites of [CR#102.1] for "player" stay.
3. **Glaring Spotlight sees through plain hexproof.** The obstacle is hexproof [CR#702.11b], and
   [CR#702.11e] extends an "as though it didn't have hexproof" permission to "hexproof from
   [quality]". Replace [CR#702.11d] with [CR#702.11b,702.11e], keeping [CR#609.4] where the
   as-though frame is cited, in `crates/core/src/deontic.rs` (the `premise = Not(Has(Hexproof))`
   bullet) and on `spotlight_sees_through_hexproof_only_for_its_controller` in
   `crates/engine/tests/stack.rs`. The other [CR#702.11d] cites, which are about hexproof-from
   itself, are correct.
4. **A redirected discard is still a discard because of [CR#614.6].** [CR#701.9c] covers only a
   discard redirected to a hidden zone. A replaced event becomes a modified event that can still
   trigger [CR#614.6], and madness "discards it, but exiles it instead" [CR#702.35a]. Where the
   exiled card is read back, cite [CR#702.35c,400.7k]. The sites are every [CR#701.9c] cite in
   `crates/core/src/event.rs`, `crates/engine/src/eval.rs`, `replace_registry.rs`,
   `resolve/action.rs`, `agenda.rs` and `step/mod.rs` that is about a discard redirected to a
   public zone. The mill half of the `agenda.rs` and `step/mod.rs` cites ([CR#701.17c], "a
   redirected card is still … milled") becomes [CR#701.17a,614.6], because [CR#701.17c] is about
   finding a milled card. Cites of [CR#701.17c] on "milled this way" reads stay.
5. **State-based destruction has two named routes.** [CR#701.8b]: destruction comes from a
   "destroy" effect or the state-based actions for lethal damage [CR#704.5g] and deathtouch damage
   [CR#704.5h]. Reword "the other is the lethal-damage SBA" (`crates/engine/src/event.rs`,
   `Cause::destroy`) and "the SBA is destruction's OTHER cause" (`crates/engine/src/sba.rs`,
   `stamp_sba_cause`) to name both state-based actions. Reword "the lethal-damage SBA's deathtouch
   clause" (`crates/core/src/filter.rs`, `crates/engine/src/step/combat.rs`) to "the
   deathtouch-damage SBA ([CR#704.5h])".
6. **One occurrence is [CR#603.2c,700.1], not [CR#603.3b].** [CR#603.3b] orders waiting triggers
   onto the stack. A mill or destroy-all batch that is one simultaneous event is [CR#608.2f] (with
   [CR#701.17a] for mill), and that one happening can be one event or several to different
   abilities [CR#603.2c,700.1]. Sites: the Mill window and mill commit in
   `crates/engine/src/step/act.rs`, the batch relocation doc in `resolve/action.rs`, and the
   dual-facet apply doc in `step/mod.rs`. The trigger-ordering cites of [CR#603.3b] stay.
7. **A departed recipient is a new object.** The `PutCounters` arm in
   `crates/engine/src/resolve/player_action.rs` cites [CR#608.2b] for "an illegal target or
   departed `This` recipient". [CR#608.2b] covers the illegal target. The departed untargeted
   recipient is unaffected because it is a new object [CR#400.7], and the quantity reads its last
   known information [CR#608.2h]. Cite [CR#608.2b,400.7], with [CR#608.2h] on the quantity clause.

## Done when

- Every site above is corrected and no comment under `crates/` still makes any of the seven claims.
- `cargo xtask cite check --list-noncompliant` is empty, `cargo xtask cite check` reports 0 stale,
  and `cargo xtask cite audit --diff` over the change is read rule by rule against each claim.

## Not in scope

- Cites in `v1/` and in `docs/tickets/done/`.
- The copy-cease and fizzle cites: `engine-spell-copy-stack-lifecycle`.

## Pointers

- The files named in each item; `data/rules/cr.txt` for the rule texts.
