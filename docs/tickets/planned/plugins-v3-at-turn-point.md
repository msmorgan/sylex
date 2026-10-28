---
needs: [plugins-v3-expansion-baseline-recursive-samples]
---
# `at_` takes a turn point; `when` and `whenever` take an event

Standard constraints apply.

## The defect

`templates/ability/{when,whenever,at_}.lean` have identical signatures and bodies: all three
take `event : GameEvent` and expand to the single `triggered` primitive. So `at_ (does it enter)`
type-checks, even though no card is printed that way.

The three trigger words all open the triggered-ability form [CR#603.1], but they do not name the
same kind of thing:

- **When** and **Whenever** take an event. The CR does not distinguish them. The choice between
  them is Oracle wording (`docs/oracle-style-guide.md`, the trigger-word guidance that begins
  "Use **When** for a discrete event that is naturally singular"). They stay two Templates over
  one constructor, each its own spelling point. That much is settled
  (`plugins-v3-primitive-template-boundary`).
- **At** names a point in the turn structure. "At the beginning of" a phase or step triggers when
  that phase or step begins [CR#603.2b]. All 52 current `at_` uses are
  `beginningOfYour`/`beginningOfPossessed`/`beginningOfEach <part>`, and one of them is wrapped in
  `nthOccurrence` (Paradox Haze, "first upkeep each turn").

## Change

- Add an authoring sort for turn points. It covers:
  - the beginning of a phase or step, with its possessor (your, each, a possessed player's);
  - end of combat;
  - the "next" / nth-occurrence forms that delayed triggers and Paradox Haze need.
- `at_` takes a turn point and expands it to the `GameEvent` it denotes. The core AST keeps one
  triggered-ability constructor, because the CR models the beginning of a step as a trigger
  event too ([CR#603.2b]).
- `when` and `whenever` refuse turn-point events.
- Rewrite the 52 `at_` uses in builtin and canon to the new argument.
- Rewrite any delayed-trigger Template that writes "at the beginning of the next …" to match.
- The spellings say "at <turn point>". Settle `at_`'s bucket-3 `TODO(spelling)` here.

## Acceptance

- `at_ (does it enter)` and `whenever (beginningOfYour upkeep)` fail to elaborate in both Lean
  and Rust, pinned by negative tests.
- Builtin and canon expansion baselines show 0 differences, apart from keys this ticket
  renames, each listed.
- Gates: `cargo xtask lean check`. Run `cargo test` and clippy for `deckmaste_semantics_v3` and
  `xtask`.
