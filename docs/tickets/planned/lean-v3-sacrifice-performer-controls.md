---
needs: [lean-retirement]
---

# Sacrifice is limited to what its performer controls

**Why.** `keyword_action/sacrifice.syl` moves its subject from `zone(battlefield, none)`, any
permanent on the battlefield. The rule limits it to the performer's own: "A player can't sacrifice
something that isn't a permanent, or something that's a permanent they don't control"
[CR#701.21a]. As written, "each opponent sacrifices a creature" can be satisfied with your
creature, and so can a bare "sacrifice a creature". The restriction cannot be written yet:
`.syl` has no way to restrict a noun phrase of any plurality by a predicate while mentioning it
once. A partitive over the subject is refused for a singular one (a Saga's
`sacrifice(thisSaga)`), and `doIf(is(subject, actorControls), move(subject, …))` mentions the
subject twice.

**Done when.**
- A primitive restricts a noun phrase, singular or plural, by a predicate with one mention of the
  noun phrase.
- `sacrifice`'s body uses it to restrict its subject to permanents the performer controls, so no
  card's noun phrase has to say "you control". The restriction is not `zone(battlefield, actor)`:
  `zone`'s second argument names an owner, and control is not ownership.
- The body reads its performer, so a Handoff ("target opponent sacrifices a creature") decides
  whose permanents are eligible.
- Everything built on sacrifice inherits the restriction (devour, exploit, champion, evoke,
  `controllerSacrifices`, every card that writes it); `forage.syl` drops its now-redundant
  `actorControls`.
- Tests that accept a sacrifice of a permanent the performer doesn't control are rewritten to
  assert the refusal, or retired.
- Everything passes the `.syl` checks.

**Not in scope.** `destroy`, which rightly reaches any permanent. The destination stays the bare
`graveyard`, which is consistent with "its owner's graveyard" [CR#701.21a].

**Pointers.** `plugins/builtin/definitions/keyword_action/{sacrifice,forage,destroy}.syl`,
`plugins/builtin/templates/action/controllerSacrifices.syl`, `docs/decisions/semantics.md` §2.3.
