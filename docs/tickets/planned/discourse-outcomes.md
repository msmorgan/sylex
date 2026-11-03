---
needs: [discourse-core]
---
# `discourse-outcomes`: action outcomes, moved-object successors, stamps, and LKI

Standard constraints apply. Outcomes and zone-change family of `semantics-rust-rewrite`: implement
action outcome publication and `thatMuch`/`thatMany` resolution, zone-change successor referents
`[CR#400.7]`, action stamps, last-known information `[CR#608.2h]`, library-shuffle forgetting, and
token/copy creation referents in `crates/deckmaste_discourse`.

## Scope & Design Decisions

1. **Action outcomes and `thatMuch` / `thatMany`
   (`[CR#107.1,107.1b]`, `Words.lean`, `Phrase.lean`)**:
   - Publish every `OutcomeSort` referent defined in `lean/Semantics/Check/Words.lean`
     (`damageDealt`, `excessDamageDealt`, `lifeGained`, `lifeLost`, `cardsDrawn`, `cardsMilled`,
     `cardsDiscarded`, `countersPut`, `countersRemoved`, `manaAdded`, `permanentsSacrificed`,
     `rollResult`, etc.) from atomic actions and life/counter deltas.
   - Resolve `Amount::ThatMuch` and `Quantity::ThatMany` against the visible outcome referent in
     scope, refusing when zero or multiple outcomes are in scope (`quantOutcomeInScope` /
     `outcomeInScope`).

2. **Zone-change successors, stamps, and shuffle forgetting (`[CR#400.7]`)**:
   - Implement `moveIntro`: moving an object to a new zone introduces a `MovedSuccessor` referent
     linked to its prior referent (`from`), stamped with destination zone, procedure `Deed`
     (`destroy`, `exile`, `sacrifice`, `discard`, `counter`, `mill`, `play`, `cast`, `return`,
     `put`), and any `MoveRider` status/designation stamps.
   - Stamp in-place actions (`tap`, `untap`, `turnFaceUp`, `turnFaceDown`, `phaseOut`,
     `regenerate`, `attach`, `unattach`, `fight`, `bite`, `gainControl`, `become`, `putCounters`,
     `removeCounters`).
   - Implement library-shuffle forgetting (`shuffledLastKnown`): shuffling a library marks prior
     library-destination moved successors so later definite/pronoun readbacks cannot track the
     card's identity in the library while still permitting LKI stat readbacks.

3. **Last-known information (`[CR#608.2h]`) and battlefield location check**:
   - Allow stat/characteristic projections (`power`, `toughness`, `manaValue`, `colors`, `types`,
     `subtypes`, `countersOn`, `controller`, `owner`) on a departed permanent referent via LKI
     `[CR#608.2h]`, while refusing in-place battlefield actions on an object that has already left
     the battlefield.
   - **Battlefield location without `isCard` on an exiled object**: Refuse `in_(battlefield)` on an
     object in exile whether or not `isCard` is conjoined (closing
     `LinkedReadback.gapBattlefieldLocationIsNotAZone` alongside
     `LinkedReadback.badLinkedObjectInAnotherZone`).

4. **Created referents (`Creation.lean`, `TokenSpec`)**:
   - Introduce `Created(k)` referents from `Action::Create` / `TokenSpec`, stamped with token
     characteristics and entry riders so subsequent clauses resolve `"that token"` /
     `"those tokens"` and `"it"` / `"them"` to the created objects.

## Tests & Verification

- **Corpus & fixture gate**: Every corpus card and fixture with no remaining `Refusal::Unsupported`
  agrees with `bindings.jsonl` and `fixtures.jsonl` (plus the documented fixes).
- **Spec catalog**: Port `docs/semantics-spec-catalog.md` Part 1 §6 (Reference scopes across zone
  changes) and Part 2 Suites B (Actor handoff, performer kind, attachments), D (Counters, tokens,
  simultaneity), and E (Damage, prevention, fights, exchanges) as native Rust tests, plus the
  refusal test for `LinkedReadback.gapBattlefieldLocationIsNotAZone`.
