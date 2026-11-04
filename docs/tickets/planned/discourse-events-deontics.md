---
needs: [discourse-outcomes]
---
# `discourse-events-deontics`: events, triggers, replacements, and deontic/modal gates

Standard constraints apply. Event, replacement, and deontic family of `semantics-rust-rewrite`:
implement `GameEvent` (`does`, `would`, `did`), trigger header contexts (including disjunctive and
coordinated trigger subjects), `Replacement` (`StepPhase.preEvent`), deontic static specs (`cant`,
`can`, `must`, `gatedBy`), modal boxes (`closeModalBox`), and duration retention in
`crates/deckmaste_discourse`.

## Scope & Design Decisions

1. **Events and trigger headers (`Events.lean`, `HeaderContext.lean`, `[CR#603.1,603.2]`)**:
   - Enforce the closed set of event-capable `Action` heads under `GameEvent::Does`, `Would`, and
     `Did` (`docs/decisions/semantics.md` §4.6), publishing event subject, actor, cause, and
     event magnitude outcomes (`lifeGained`, `damageDealt`, `cardsDrawn`, etc.) into the
     trigger/lookback `DRS`.
   - Implement `commonHeaderContext` for disjunctive trigger headers (`GameEvent::Or`:
     `"Whenever X or Y"`), publishing the shared referent signature visible to the trigger effect.
   - **Coordinated trigger subject anaphora** (*Pine Walker*): When a trigger event's subject is a
     `NounPhrase::Or` / `NounPhrase::And` coordination (`"Whenever this creature or another
     creature you control is turned face up, untap that creature"`), publish a merged subject
     referent representing whichever coordinated branch matched the event so `"that creature"`
     resolves to it; respell `plugins/canon/cards/Pine Walker.syl` to use the printed
     coordination and `"that creature"`.

2. **Replacements (`Replacement`, `StepPhase.preEvent`, `[CR#614.1,614.6]`)**:
   - Step `StaticSpec::Replace` by evaluating the replaced `would` event in `StepPhase.preEvent`
     (so the replacement body reads the event's announced subjects, destinations, and magnitudes
     before the replaced event takes effect, without seeing post-event moved successors from the
     replaced event itself), then step `instead`, `before`, `alongside`, or `withRiders` in
     `StepPhase.postEvent`.
   - Enforce self-replacement and entry-rider restrictions (`[CR#614.12,614.15]`).

3. **Deontics, modal boxes, and duration retention (`[CR#101.2,601.2,611.2]`)**:
   - Enforce the closed set of deontic-capable `Action` heads under `StaticSpec::Cant`, `Can`,
     `Must`, and `GatedBy` (`docs/decisions/semantics.md` §4.7).
   - Implement `closeModalBox` on deontic and conditional scopes so referents introduced inside a
     prohibition or modal possibility do not leak as guaranteed discourse referents to subsequent
     indicative sentences (except where explicitly published by a conjunction rule such as
     `discourse-cast-permission-publishes-spell`).
   - Check duration retention (`until`, `forAsLongAs`, delayed triggers `[CR#603.7]`).

## Tests & Verification

- **Corpus & fixture gate**: Every corpus card and fixture with no remaining `Refusal::Unsupported`
  agrees with `bindings.jsonl` and `fixtures.jsonl` (plus the documented fixes).
- ***Pine Walker* respelling**: `plugins/canon/cards/Pine Walker.syl` is respelled with the
  coordinated trigger subject and `untap(that(isCreature))`, and checks with zero refusals.
- **Spec catalog**: Port `docs/semantics-spec-catalog.md` Part 2 Suites F (Triggers, event
  patterns), I (Turn structure, reflexive triggers, timing), J (Deontic rules), and O (Static
  abilities) as native Rust tests.
