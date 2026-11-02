---
needs: [semantics-rust-rewrite]
---
# `discourse`: a Definition parameter's argument resolves against all its uses

Found 2026-11-02 while trying `lean-v3-sacrifice-performer-controls` in Lean, traced in
`lean/Semantics/Check/Phrase.lean` (not probed). The Lean checker has two defects; `discourse` must
not reproduce them.

A parameter used twice in a Definition's body compiles to a `firstMention` and a `laterMention`.
The caller's argument (say `it`) is resolved once, at the first mention, in that mention's slot;
the later mention reuses the binding without checking it against its own slot.

1. Which use comes first decides what the argument means. Written
   `doIf(is(subject, actorControls), move(subject, …))`, sacrifice's first mention is the `is`
   condition, which narrows a bare pronoun only for a predicate that demands a card. So `it`
   resolves as a bare pronoun of any kind, while the move's subject slot would have narrowed it to
   a permanent (`.atSlot .permanent`). Reordering a Definition's body can change what a card's
   "it" refers to.
2. A later mention is not checked against its slot: a referent that is not a permanent is moved
   off the battlefield without a refusal.

## Requirement

- A parameter's argument is resolved against the constraints of all the parameter's uses in the
  body (at least its action slots), not against whichever use is first.
- Every use is checked against its own slot.

## Tests

- `sacrifice it` with no permanent in scope (Zone.lean's `badInterceptReplacementAntecedent`, and
  after `draw 1`) is refused `anaphor (.atSlot .permanent) .one 0` whether sacrifice's body names
  its subject once or as `doIf(is(subject, …), move(subject, …))`.
- The same holds with the two uses in either order.
