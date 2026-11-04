---
needs: [core-as-enters-choices, core-regions-substrate, core-regions-discourse, core-regions-costs-and-captures, core-regions-witness-fixtures, core-regions-test-restoration, core-regions-discourse-closeout, core-regions-captures-and-memory, engine-granted-body-captures, engine-cross-target-announce-enumeration, engine-damage-fact-participant-lki, lowering-pay-cost-decision-registers, engine-derived-reference-in-filters, engine-cross-target-retarget-and-precheck, engine-cast-from-zones, engine-condition-lki-context, engine-deontic-legality-residue, engine-departed-reference-fizzle, engine-granted-prevention-rows, engine-granted-static-rows, engine-piles, engine-snapshot-combat-state, engine-snapshot-object-classification, engine-statof-reference-channel, engine-turn-modification]
---
**Epic: land every engine mechanism that faithfully lowered Foundations cards
need.** An audit of all 517 distinct English card names in
`data/mtgjson/AllSetFiles/FDN.json` found 38 cards whose faithful core terms
reach a production panic and 17 more that trip a debug assertion. Each of those
aborts marks a missing mechanism. The missing mechanism is the defect; the
loud failure is correct until that mechanism lands
([Engine](../../decisions/engine.md) law 6).

This is a closure node, not an implementation bucket. Its `needs:` are the
mechanism tickets that must land; the table records the card witnesses so the
set-level claim stays reproducible.

| Missing engine mechanism | Owning ticket | Foundations witnesses |
|---|---|---|
| Partition/choose-pile resolution | [[engine-piles]] | Curator of Destinies |
| Turn-structure insertion | [[engine-turn-modification]] | Aurelia, the Warleader |
| Duration-created prevention rows | [[engine-granted-prevention-rows]] | Fleeting Flight |
| Duration-created player/static rows | [[engine-granted-static-rows]] | Finale of Revelation |
| Decision-capable as-enters replacement and stored value | [[core-as-enters-choices]]; routing provenance in [[engine-as-enters-fold-breadth]] | Diamond Mare; Heraldic Banner; Sorcerous Spyglass; Uncharted Haven |
| Cast/play permissions with a source zone or cost override | [[engine-deontic-legality-residue]], [[engine-cast-from-zones]] | Muldrotha, the Gravetide; Omniscience; Quilled Greatwurm; Tinybones, Bauble Burglar; Vizier of the Menagerie |
| Snapshot object kind/subtype predicates | [[engine-snapshot-object-classification]] | Arahbo, the First Fang; Crossway Troublemakers; Gate Colossus; Gateway Sneak; High-Society Hunter; Infernal Vessel; Kalastria Highborn; Lathliss, Dragon Queen; Midnight Reaper; Spinner of Souls; Valkyrie’s Call; Wildborn Preserver; Youthful Valkyrie |
| Snapshot combat-state predicates | [[engine-snapshot-combat-state]] | Garna, Bloodfist of Keld |
| Snapshot candidate/reference bindings | [[core-regions-substrate]] | Predator Ooze |
| Live/LKI-aware `StatOf` | [[engine-statof-reference-channel]] | Heartfire Immolator; Halana and Alena, Partners; Ovika, Enigma Goliath; Prime Speaker Zegana; plus the gone-reference side of Bite Down and Felling Blow |
| Root additional-cost hoisting and paid-object binding | [[core-regions-costs-and-captures]] | Ayli, Eternal Pilgrim |
| Departed action operands after partial target loss/source departure | [[engine-departed-reference-fizzle]] | Bite Down; Drakuseth, Maw of Flames; Felling Blow; Heroes’ Bane |
| Current-state conditions must not accidentally consume LKI | [[engine-condition-lki-context]] | Affectionate Indrik |
| Full frame in candidate matching | [[core-regions-substrate]] | Angel of Finality; Arbiter of Woe; Blasphemous Edict; Bloodtithe Collector; Burglar Rat; Deadly Brew; Duress; Fiery Annihilation; Liliana, Dreadhorde General; Painful Quandary; Perforating Artist; Pilfer; River’s Rebuke; Run Away Together; Steel Hellkite; Tribute to Hunger; Trygon Predator. Tinybones also reaches this family after its deontic row is consumed. |

Done means:

1. Every row's mechanism is built, with behavioral engine coverage for its
   distinct binding and liveness shapes, using direct core fixtures where card
   authoring is not present.
2. Each witness performs the card-required behavior. A panic is never removed
   by turning a legal term into a silent no-op; it goes away only because the
   mechanism behind it now exists.
3. Partial-target and departed-source tests assert the correct remaining
   effects, not only the absence of a panic.
4. A replay of the audit's faithful core terms against the same 517-name
   snapshot reaches no abort. Any newly exposed abort is routed to the
   mechanism ticket that owns it before this epic moves to `done/`.

Explicitly outside this epic: completeness of builtin macro bodies, parser or
authoring convenience, and non-panicking semantic gaps. Crew is not a missing
engine mechanism here: aggregate-stat tap costs and the temporary type-change
row already execute; an empty `Crew` macro body is authoring work.
