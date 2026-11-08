---
needs: []
---
# Monstrosity and adapt name their Source

`monstrosity.syl` and `adapt.syl` read their "this permanent" as `actor` under
`permanent = isAbout`, so a Handoff ("target creature adapts 2") renames it. No card prints such a
Handoff. Both rules are stated over "this permanent" and say nothing about who performs them:
"If this permanent isn't monstrous, put N +1/+1 counters on it and it becomes monstrous"
[CR#701.37a], "If this permanent has no +1/+1 counters on it, put N +1/+1 counters on it"
[CR#701.46a]. The controller follows them as any instruction [CR#608.2c,701.37c].

## Change

- Both bodies name `thisPermanent` instead of `actor`, and drop `permanent = isAbout`. They become
  ordinary actions the controller performs, like scry.
- Retire `DeedPermanent.isAbout` and `Deed.aboutAPermanent`, with the checker's handling of them
  (the `.ownScope` frame and `clauseActorKind`). Monstrosity and adapt are their only users.
- Add no Handoff refusal for them. "Target opponent adapts 2" checks as that opponent following
  the instruction, its "this permanent" still the Source; no card writes it.
- Leave `support.syl` and `theRingTemptsYou.syl` as they are. With no other anchor in scope,
  support's `isOther` contrasts with the Source, so on an instant or sorcery every creature is
  "other", which is [CR#701.41a]'s second meaning without a split by card class.

## Proofs

- Retire `badAdaptHandedToOpponent`, `okAdaptHandedToAPermanent` and
  `badMonstrosityHandedToOpponent` (`Proofs/Actor.lean`): they assert a Handoff reading no card
  prints.
- Retire `biomancersFamiliarWatchesTheCreatureThatAdapts` (`Proofs/CardsCounters.lean`): it tests
  whether one card's pattern catches the events another causes, which is event matching, not
  meaning. "Target creature adapts" moves to `syl-keyword-verb-events`.
- `badPlayersCantExploreOrAdapt` keeps its explore half; its adapt half moves to
  `syl-keyword-verb-events`, where the event's subject kind is declared.
- `Tables.permanentPerformedActions` drops its `aboutAPermanent` conjunct and lists adapt and
  monstrosity among the deeds no permanent performs.
- Keep `bareAdaptIsTheControllers` and `docSamsonWatchesAdaptsCounters`.
- Add a proof that `support 2` checks on an instant.

## Docs

- `lean/CONTRACTS.md`, "The performer and the handoff": adapt and monstrosity name their Source and
  the controller follows them; no Handoff renames it.
- `lean/Semantics/Words.lean`: remove `isAbout` and its doc; the `Deed.action` doc no longer
  mentions it.
- `docs/decisions/semantics-v3.md` §2.1: record the reading for monstrosity and adapt (adding adapt
  to the list itself is `docs-keyword-classification-corrections`).

## Settled

- Lean's current plumbing is no authority on the representation, and nothing here may be a
  per-definition special case in `.lean`.
- "<X> adapts" is not an inflection of the keyword action. It is an event defined as evolve's is
  [CR#702.100b], in `syl-keyword-verb-events`.
- An earlier attempt is archived at `archive-monstrosity-actorless`; only its `monstrosity.syl`
  body carries over.
