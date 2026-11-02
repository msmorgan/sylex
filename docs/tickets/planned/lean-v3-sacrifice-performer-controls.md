---
needs: [lean-retirement]
---
# Sacrifice is limited to what its performer controls

`keyword_action/sacrifice.syl` moves its subject from `zone(battlefield, none)`, any permanent on
the battlefield. The rule limits it to the performer's own: "To sacrifice a permanent, its
controller moves it from the battlefield directly to its owner's graveyard. A player can't
sacrifice something that isn't a permanent, or something that's a permanent they don't control"
[CR#701.21a]. As written, "each opponent sacrifices a creature" can be satisfied with your creature,
and so can a bare "sacrifice a creature".

## Change

- Restrict the sacrificed subject to permanents the performer controls, in the Definition's body,
  so no card's noun phrase has to say "you control". `Location.zone`'s second argument names an
  owner ("your graveyard"), and control is not ownership, so the restriction is not
  `zone(battlefield, actor)`.
- Settled (2026-11-02, after a Lean attempt): the AST has no form that restricts a noun phrase of
  any plurality by a predicate while mentioning it once. A partitive over the subject is refused
  for a singular one (`partitiveBase`; a Saga's `sacrifice(thisSaga)`), and
  `doIf(is(subject, actorControls), move(subject, …))` mentions the subject twice and exposed
  `discourse-parameter-argument-resolution`. Add that restrictor in Rust `semantics`, then write
  sacrifice with it: one mention of the subject.
- The body then reads its performer, so a Handoff ("target opponent sacrifices a creature") binds
  whose permanents are eligible.
- Bare `graveyard` is `zone(graveyard, none)`, a bare zone name; nothing in the model contradicts
  "its owner's graveyard" [CR#701.21a], so it stays.
- Everything built on sacrifice inherits the restriction: devour's, exploit's and champion's
  sacrifices, evoke's, `controllerSacrifices`, and every card that writes it. `forage.syl`'s own
  `actorControls` in its sacrifice argument becomes redundant; drop it. Tests that accept a
  sacrifice of a permanent the performer doesn't control are wrong and are rewritten to assert the
  refusal or retired.

`destroy.syl` has the same origin and is right as it is: anything can destroy any permanent.
