---
needs: []
---
# "Unblocked" is a status a creature gets at the declare blockers step, and block triggers fire per [CR#509.3a..509.3d]

## Why

`StatePredicate::Unblocked` matches "attacking and not blocked", so it is true right after
attackers are declared. An attacker becomes unblocked only when blockers are declared with none
for it [CR#509.1h], or when it enters attacking during declare blockers or later [CR#508.4d];
ninjutsu and every other "unblocked attacker" cost is payable a step early (Ninja of the Deep
Hours' ruling: "Before then, attacking creatures are neither blocked nor unblocked.").

Separately, `scan_triggers` drops every `Blocked` fact after the first per attacker before any
watcher sees it. Two bushido creatures blocking one attacker trigger only one bushido, and a
creature with "whenever this creature blocks" blocking two attackers triggers twice instead of
once. The comment there claiming the [CR#509.3d] pattern shape does not exist is stale.

## Design

- `CombatState` keeps an explicit unblocked set beside its blocked set; "unblocked" is never
  derived as "attacking and not blocked", since before blockers are declared an attacker is
  neither. `StatePredicate::Unblocked` and every later derivation of "unblocked" read the set.
- Every `Blocked` fact of a declaration reaches the trigger scan. Block triggers deduplicate per
  watching ability, keyed by the `BlockDeclared` pattern it uses [CR#700.1]:

  | Pattern | Wording | Key | CR |
  |---|---|---|---|
  | `of` only | "becomes blocked" | (watcher, ability, attacker) per declaration | [CR#509.3c] |
  | `by` only | "blocks" | (watcher, ability, blocker) | [CR#509.3a] |
  | `by` and `of` | "blocks a creature", "becomes blocked by a creature" | none: once per pair | [CR#509.3b,509.3d] |
  | union of `by` and `of` | Bushido's "blocks or becomes blocked" | (watcher, ability, creature) | [CR#702.45a] |

## Done when

- The declare blockers turn-based action puts every attacker with no blocker declared into the
  unblocked set [CR#509.1h], and a creature put onto the battlefield attacking during declare
  blockers or later enters it [CR#508.4d]. It stays unblocked until it is removed from combat,
  an effect says it becomes blocked (which moves it to the blocked set), or the combat phase
  ends; an effect that says an attacker becomes unblocked puts it in the set [CR#509.1h].
- `scan_triggers` no longer drops `Blocked` facts; block triggers deduplicate by the Design
  table, and the stale comment is gone.
- Tests, with real cards:
  - `Unblocked` is false after attackers are declared and true after blockers are declared for
    an attacker with no blocker; `combat_filters_read_combat_state` is rewritten to assert this.
  - Two Samurai of the Pale Curtain blocking one attacker: both bushido abilities trigger.
  - A creature blocking two attackers: Folk of An-Havva's "blocks" triggers once, Wall of Frost's
    "blocks a creature" twice (a minimal repro granting the extra block is fine).
  - Deepwood Tantiv's "becomes blocked" still triggers once when blocked by two creatures.

## Pointers

- `crates/engine/src/combat.rs`: `CombatState`.
- `crates/engine/src/target.rs`: the `StatePredicate::Unblocked` arm,
  `combat_filters_read_combat_state`.
- `crates/engine/src/trigger.rs`: `scan_triggers` (`blocked_attackers`).
- `crates/engine/src/eval.rs`: `EventFilter::BlockDeclared`.
- `crates/engine/tests/combat.rs`: the Deepwood Tantiv test.
- `plugins/builtin/definitions/keyword_ability/bushido.syl`, `ninjutsu.syl`.
- `docs/tickets/maybe/engine-lookback-occurrence-watchers.md`: touches the same loop.
