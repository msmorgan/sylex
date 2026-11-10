---
needs: [plugins-v3-data-language, lean-retirement, syl-event-disjunction]
---

# `at` takes a turn point; `when` and `whenever` take an event

## Why

`plugins/builtin/templates/ability/{at,when,whenever}.syl` have the same signature and
body: each takes `event: GameEvent` and expands to the one `triggered` primitive, so `at(does(it,
enter), …)` resolves though no card is printed that way. The three words all open a triggered
ability [CR#603.1], but they name different things. "When" and "whenever" take an event; the CR
does not tell them apart, so they stay two Templates over one constructor, chosen by Oracle
wording (`docs/oracle-style-guide.md`, "Use **When** for a discrete event"). "At" names a point
in the turn: "at the beginning of" a phase or step triggers when it begins [CR#603.2b]. Every
current `at` call is a `beginningOf…` event, one wrapped in `nthOccurrence` (*Paradox Haze*,
"first upkeep each turn").

## Done when

- A turn-point sort covers the beginning of a phase or step with its possessor (your, each, a
  possessed player's), end of combat, and the "next" and nth-occurrence forms that delayed
  triggers and *Paradox Haze* need.
- `at` takes a turn point, is spelled "At <turn point>, <action>", and expands to the event the
  turn point denotes; there is still one triggered-ability constructor, because the CR treats a
  step's beginning as a trigger event [CR#603.2b]. Its base spelling is settled; the
  `TODO(spelling)` for its limit, if and joins clauses moves to `syl-spelling-optional-segments`.
- `when` and `whenever` refuse a turn point as their own header's event. A joined header may be
  a turn-point header beside an event header, each `TriggerHeader` printing its own trigger
  word: *Minsc & Boo, Timeless Heroes* ("When Minsc & Boo enters and at the beginning of your
  upkeep, …") is writable.
- Every `at` call in builtin and canon, and the delayed-trigger Templates that write "at the
  beginning of the next …" (`atNext`, `beginningOfNext`, `asThoughFlashSacrificedAtCleanup`), use
  the new argument.
- Negative tests pin that `at(does(it, enter), …)` and `whenever(beginningOfYour(upkeep), …)` are
  refused.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- *Kitsune Mystic* prints "At the beginning of the end step", while canon writes `beginningOfEach`,
  spelled "each".
- A delayed "at" trigger with a stated duration (epic, paradigm) is `delayWithin(reached(…))`,
  whose spelling is still "When …".

## Pointers

`plugins/builtin/templates/ability/{at,when,whenever}.syl`,
`plugins/builtin/templates/event/beginningOfNext.syl`, `plugins/builtin/templates/action/atNext.syl`,
`docs/decisions/semantics.md` §4.6.
