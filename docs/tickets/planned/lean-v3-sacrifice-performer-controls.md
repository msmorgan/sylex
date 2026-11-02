---
needs: []
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
  so no card's noun phrase has to say "you control". Write it as a control restriction keyed to the
  performer (`actorControls`, as amass and bolster use), not as `zone(battlefield, actor)`:
  `Location.zone`'s second argument names an owner ("your graveyard"), and control is not
  ownership.
- The body then reads its performer, so a Handoff ("target opponent sacrifices a creature") binds
  whose permanents are eligible.
- Check that bare `graveyard` resolves to the permanent's owner's graveyard, as the rule says;
  fix it if it does not.
- Everything built on sacrifice inherits the restriction: devour's, exploit's and champion's
  sacrifices, evoke's, `controllerSacrifices`, and every card that writes it. Proofs that assert a
  sacrifice of a permanent the performer doesn't control checks are wrong and are rewritten to
  assert the refusal or retired.

`destroy.syl` has the same origin and is right as it is: anything can destroy any permanent.
