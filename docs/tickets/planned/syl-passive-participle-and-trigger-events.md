---
needs: [cleanup-step6-spine, syl-clause-and-imperative, syl-union-membership]
specs: [voice-and-object-phrases, kind-sorts-and-position-unions]
---

# The passive is `done` and `be`, and a trigger event is a subject and a verb phrase

## Why

Voice is carried outside the verb phrase, by a `Subject` word that is not the printed subject:
`actor(np)`, the agentless `any`, or `by(np)`. A trigger's event is `does(subj: Subject, action)`
(306 calls in `plugins/canon/`, 93 in `plugins/builtin/`, 54 of them `does(any, …)`, by `grep`).
So:

- The printed subject of a passive is buried in the verb phrase: *Grollub*'s "Whenever this
  creature is dealt damage" is `does(any, dealDamage(recipient = this(creature)))`.
- A verb with two passives has one term for both. *Grollub* promotes the recipient and
  *Phytohydra*'s "If damage would be dealt to this creature" promotes the damage, and both hold
  the same `dealDamage(recipient = this(creature))` under `any`.
- A passive cannot coordinate with an active under one subject (*Kaya's Ghostform*'s "When
  enchanted permanent dies or is put into exile"), or stand as a participle (*Abu Ja'far*'s "all
  creatures blocking or blocked by it").
- Some sites write a passive the card does not print: *Captain Marvel, Apex Avenger*'s "Whenever
  you put one or more counters on another creature" and *Divine Intervention*'s "When you remove
  the last intervention counter from this enchantment" are each `does(by(you), …)`.
- `becomeBlocked` and `resolve` print their own subject and leave the event's `any`:
  `does(any, becomeBlocked(this(creature)))` (*Chub Toad*, `bushido`, `afflict`, `rampage`),
  `does(any, resolve(this(spell)))` (`buyback`, `haunt`, `rebound`; `paradigm` calls `resolve` the
  same way).

The spec's §2 to §4 replace this; this ticket builds the passive words and the trigger event,
and the tickets after it the other words that take a `Subject`.

## Done when

- `done(phrase: VerbPhrase, by: Option(NounPhrase) = none): VerbPhrase` and
  `be(phrase: VerbPhrase): VerbPhrase` exist, and the word `subject` stands in the slot a `done`
  phrase promotes.
- `does(subject: NounPhrase, phrase: VerbPhrase): GameEvent` is the trigger event, worded as
  `clause` is, and no `does` call holds `actor(…)`, `any` or `by(…)`.
- Each former `does(any, …)` and `does(by(…), …)` site is read from its card's Oracle line or its
  rule's text, and the landing lists each with that line and the form it took:
  - a printed passive is `be(done(…))` said of the printed subject, with `subject` in the slot
    the line promotes;
  - a printed active is written active (*Captain Marvel, Apex Avenger*, *Divine Intervention*);
  - an event printed in the active whose subject performs nothing is its Template or Definition
    said of that subject (spec §4);
  - a rule body's pattern that no card prints writes `any`, the wildcard word at `NounPhrase`,
    as its subject (spec §8).
- The `does(actor(x), …)` sites, whose subject only loses its wrapper, are rewritten by a
  one-off script, kept in `docs/evidence/`, which checks that each is the term before it without
  the wrapper; the landing reports the counts it compared.
- `deal`'s second passive has its Template `dealIndirect(recipient, damage)`, written only under
  `done`: *Grollub*'s event is `does(this(creature), be(done(dealIndirect(subject, bare(damage)))))`.
- `becomeBlocked(by: Option(NounPhrase) = none)`, worded "become blocked< by $by>", and `resolve`
  take no subject, and neither do `becomeBlocking`, `stopBlocking` and `phaseOutUntil`, which
  nothing calls. *Chub Toad*, `bushido`, `afflict`, `rampage`, `buyback`, `haunt`, `paradigm` and
  `rebound` are rewritten, and *Aisling Leprechaun*'s and *Inferno Elemental*'s
  `does(by(a(isCreature)), block(attacker = this(creature)))`, for "becomes blocked by a creature",
  is `becomeBlocked(by = …)` said of this creature [CR#509.3d].
- `transformSelf` exists, a Template over the Definition `transform` with `subject` as its
  argument, and *Cult of the Waxing Moon*'s "Whenever a permanent you control transforms into a
  non-Human creature" calls it.
- `regeneration` takes no subject parameter and reads `subject`, and *Matopi Golem*'s "When it
  regenerates this way" is `regeneration` said of the creature, where the card writes
  `does(any, regeneration(this(creature)))` today. Which declaration carries the Deed stays
  `syl-explicit-deeds`'s.
- *Tide of War* is in `plugins/canon/cards/`, its "each blocking creature is sacrificed by its
  controller" a clause over `be(done(sacrifice(subject), by = …))`, checked against its Oracle
  text. An expansion test (`crates/xtask/src/dsl_expand/tests.rs`) shows a Definition's call
  under `done` binds `$actor` to `by`, and to nothing the term names where `by` is absent.
- `cargo xtask dsl check` refuses `subject` in a card's term outside a `done` phrase and a `done`
  phrase that does not hold exactly one `subject`, with a negative test for each; a Template's
  body may place `subject` wherever a noun phrase stands. The five canon cards that write the
  participant word in their own term (*Afflicted Deserter // Werewolf Ransacker*, *Bewitching
  Leechcraft*, *Darksteel Garrison*, *Tuktuk Scrapper*, *Verity Circle*) are rewritten.
- `docs/design/semantics.md` §2.1, `docs/design/surface.md` §7.2 and §7.4, `docs/design/syl.md` §4,
  `docs/design/spelling.md` §1, §2, §4, §5, §6, §7, §9 and §10 and
  `docs/design/architecture.md` say what the spec's §10 table gives for each: `done` selects the
  passive, and nothing describes passive subject promotion or a `Subject` that carries voice. `surface.md` §7.1's table has the
  trigger row of the spec's §1 table. The glossary entries **Subject**, **Actor** and **Patient**
  (`docs/contexts/game-model/CONTEXT.md`) are rewritten as the spec's §10 says.
- Each fixed participle at `Participle` (`attacking`, `blocking`, `blocked`, `chosen`,
  `equipped`, `foretold`, `attachedTo`, `named`, `notChosen`, `notNamed`, `was`,
  `controlledBy`) is its verb phrase standing in `pre` or `post`; `VerbPhrase` replaces
  `Participle` as the member of `PreModifier` and `PostModifier`, with the form stated on the
  membership, and `Participle` is gone; the noun and the verb `copy`, the verb and the test
  `attachment`, and the verb and the noun `counter` have different names; and the passives at `Predicative` (`attachedTo`,
  `attachedBy`, `controlledBy`, `named`, `was`) and the relatives over a progressive
  (*Propaganda*'s "that's attacking you", *Verity Circle*'s "isn't being declared as an
  attacker") are composed from their verbs, each leaving the spec's list of declarations that
  stay at a position sort
  (`docs/specs/kind-sorts-and-position-unions.md` §2, §4).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- `would`, `did`, the tallies and the deontic words, which keep their `Subject` until
  `syl-would-operator-and-replacement-clauses`, `syl-lookback-operator-and-holds` and
  `syl-deontic-verb-phrase-operators`; the last deletes the sort.
- One `does` over an `or` of verb phrases ("blocks or becomes blocked", "dies or is put into
  exile") and the shared "blocks or becomes blocked by a creature":
  `syl-shared-phrase-written-once`. This ticket leaves an event `or`'s arms each with its subject.
- The agent printed anywhere but last among a participle's complements (*Suffocation*, *Impact
  Resonance*), negation over `be`, and the perfect: not designed (spec §11, §13).
- Which heads a `does` admits: `syl-event-gate`.

## Pointers

- `docs/specs/voice-and-object-phrases.md` §2, §3, §4, §9, §10
- `plugins/builtin/primitives/event/does.syl`,
  `plugins/builtin/primitives/noun/{actor,by}.syl`, `plugins/builtin/primitives/subject/any.syl`,
  `plugins/builtin/templates/action/{becomeBlocked,becomeBlocking,stopBlocking,phaseOutUntil,becomeStatus,die}.syl`,
  `plugins/builtin/definitions/keyword_action/{resolve,transform,sacrifice}.syl`
- `crates/xtask/src/dsl_check.rs`, `crates/xtask/src/dsl_expand/tests.rs`
- `docs/design/semantics.md` §2.1; `docs/design/surface.md` §7.1, §7.2, §7.4;
  `docs/design/spelling.md` §2
