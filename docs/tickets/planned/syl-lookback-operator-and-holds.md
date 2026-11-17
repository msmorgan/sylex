---
needs: [cleanup-step6-spine, syl-passive-participle-and-trigger-events, syl-union-membership]
specs: [voice-and-object-phrases, kind-sorts-and-position-unions]
---

# A lookback is the operator `did`, and a condition stated as a clause is `holds`

## Why

A lookback is written four ways: `did(subj: Subject, action, within)` at `Condition` and at
`Predicate`, and the agentless `was(action, within)` at each (`did` is called 83 times in
`plugins/canon/` and 23 in `plugins/builtin/`, `was` 12 and 23, by `grep`). Each carries its
voice in a `Subject` word, so the printed subject and voice are not the term's:

- *Sengir Vampire* prints a passive participle with its agent, "a creature dealt damage by this
  creature this turn", and writes an active clause,
  `did(actor(this(creature)), dealDamage(recipient = which), …)`.
- *Phyrexian Rebirth*'s "the number of creatures destroyed this way" is `was(destroy(which),
  within = thisWay)`, a second word for the past because the first has no passive.
- *Tuktuk Scrapper*'s "If that artifact is put into a graveyard this way" needs the participant
  word in the card's own term, `did(actor(that(artifact)), putInto(actor, graveyard), thisWay)`.

The tallies (`tally`, `eventCount`, `eventSum`, `eventTally`) and the `DidClause` record hold the
same `Subject` and verb phrase.

## Open questions

- **Where a lookback stands after a head.** The spec's §8 puts `did(phrase, within)`, a
  `VerbPhrase`, in a nominal's `post` ("creatures that attacked this turn") and writes
  "creatures destroyed this way" as `did` over a `done` phrase. A verb phrase is no member of
  `PostModifier`, and `done` produces a `Participle`
  (`docs/specs/kind-sorts-and-position-unions.md` §2). "That attacked this turn" is a relative
  clause and "destroyed this way" a participle with its span; what each is written with.
- **"If it was blocked this turn"** (*Fyndhorn Druid*), written
  `was(becomeBlocked(it), within(soFarThisTurn))` today. Two terms are well sorted, and the spec
  picks neither (§9): the passive of `block`, `holds(it, did(be(done(block(attacker =
  subject))), …))`, which asks whether a creature blocked it; and a lookback over
  `becomeBlocked`, the Blocked status, which an effect can give with no blocker [CR#509.1h], and
  which prints "became blocked" unless a Template words it "was blocked". The card has no
  ruling. Recommendation: the lookback over `becomeBlocked` under a Template for the wording,
  since the card asks whether the creature was a blocked creature.

## Done when

- The open question is answered.
- `did(phrase: VerbPhrase, within: Option(LookbackSpan) = none): VerbPhrase` is the lookback,
  keeping the `ordinal` a description takes today, and
  `holds(subject: NounPhrase, phrase: VerbPhrase): Condition` states a condition as a clause,
  worded as `clause` is. *Efflorescence*'s "If you gained life this turn" is a `holds` over a
  `did`, with "you" its subject.
- As a description, `did` stands in a nominal's postmodifier list, said of the thing described,
  the way `syl-nominal-descriptions` settles for a verb phrase there; a relativized object keeps
  `which` (*Alabaster Leech*'s "White spells you cast").
- `did` is gone at `Predicative`
  (`docs/specs/kind-sorts-and-position-unions.md` §5). `was` is gone at `Condition` and at `Predicate`, and no lookback holds `actor(…)`, `any` or
  `by(…)`. Each former `was(…)`, `did(any, …)` and `did(by(…), …)` site, and each `did(actor(…),
  …)` whose card prints a participle, is read from its Oracle line: *Sengir Vampire*'s
  description is a `did` over `done(dealIndirect(subject, …), by = this(creature))`, *Phyrexian
  Rebirth*'s a `did` over a `done` phrase, *Tuktuk Scrapper*'s condition a `holds` over the
  passive of `putInto`. A rule body's pattern that no card or rule prints writes `any`, the
  wildcard word at `NounPhrase`, as its subject (spec §8). The landing lists each site with its
  line.
- The `did(actor(x), …)` sites whose card prints an active clause (63 `did(actor(…), …)` calls
  today) are rewritten by a one-off script, kept in `docs/evidence/`, which checks that each
  carries the same subject, verb phrase, window and ordinal before and after; the landing reports
  the counts it compared.
- The `subj` of `DidClause`, `tally`, `eventCount`, `eventSum` and `eventTally` is a
  `NounPhrase`, and their passive sites are written with `done`. `happened` and
  `predicateDid` go if nothing calls them.
- `docs/design/semantics.md` §3.4 states the lookback as the spec's §8 does, and
  `docs/design/surface.md` §7.1's table has the lookback and condition rows of the spec's §1
  table.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The `holds` at `AsThough` (`primitives/static/holds.syl`, "as though $cond"): another word, at
  a sort no coercion joins to `Condition`.
- The wording of the tallies, the perfect ("has been dealt damage this turn", *Bloodcrazed
  Goblin*) and negation over `be` ("attacks and isn't blocked"): not designed (spec §11).
- Where a window prints among the verb's other dependents (*Suffocation*'s "dealt damage this
  turn by …"): not designed (spec §13); a window stays `did`'s `within`.
- Which heads a `did` admits: `syl-event-gate`.

## Pointers

- `docs/specs/voice-and-object-phrases.md` §1, §3, §8, §11
- `plugins/builtin/templates/{condition,predicate}/{did,was}.syl`,
  `plugins/builtin/primitives/condition/happened.syl`,
  `plugins/builtin/primitives/predicate/predicateDid.syl`,
  `plugins/builtin/primitives/quantity/tally.syl`,
  `plugins/builtin/templates/amount/{eventCount,eventSum,eventTally}.syl`
- `crates/xtask/src/dsl_check/schema.rs` (`DidClause`)
- `plugins/canon/cards/{Sengir Vampire,Phyrexian Rebirth,Tuktuk Scrapper,Fyndhorn Druid,Alabaster Leech,Efflorescence}.syl`
- `docs/design/surface.md` §7.1; `docs/design/semantics.md` §3.4
