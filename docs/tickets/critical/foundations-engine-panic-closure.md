---
needs: [core-as-enters-choices, core-regions-substrate, core-regions-discourse, core-regions-costs-and-captures, core-regions-witness-fixtures, core-regions-test-restoration, core-regions-discourse-closeout, core-regions-captures-and-memory, engine-granted-body-captures, engine-cross-target-announce-enumeration, engine-damage-fact-participant-lki, lowering-pay-cost-decision-registers, engine-derived-reference-in-filters, engine-cross-target-retarget-and-precheck, engine-cast-from-zones, engine-condition-lki-context, engine-deontic-legality-residue, engine-departed-reference-fizzle, engine-granted-prevention-rows, engine-granted-static-rows, engine-piles, engine-snapshot-combat-state, engine-snapshot-object-classification, engine-statof-reference-channel, engine-turn-modification]
---

# Epic: every engine mechanism that faithfully lowered Foundations cards need is built

## Why

An audit of the 517 distinct English card names in `data/mtgjson/AllSetFiles/FDN.json` found
cards whose faithful core terms reach a production panic or a debug assertion in the engine.
Each abort marks a missing mechanism. The missing mechanism is the defect, and the loud failure
is correct until the mechanism lands ([Engine](../../decisions/engine.md) law 6). This ticket
closes when those mechanisms exist; its `needs:` are the mechanism tickets, and the table keeps
the card witnesses for the ones still open.

| Missing engine mechanism | Owning ticket | Foundations witnesses |
|---|---|---|
| Partition/choose-pile resolution | `engine-piles` | Curator of Destinies |
| Turn-structure insertion | `engine-turn-modification` | Aurelia, the Warleader |
| Duration-created prevention rows | `engine-granted-prevention-rows` | Fleeting Flight |
| Duration-created player/static rows | `engine-granted-static-rows` | Finale of Revelation |
| Decision-capable as-enters replacement and stored value | `core-as-enters-choices`; routing in `engine-as-enters-fold-breadth` | Diamond Mare; Heraldic Banner; Sorcerous Spyglass; Uncharted Haven |
| Cast/play permissions with a source zone or cost override | `engine-deontic-legality-residue`, `engine-cast-from-zones` | Muldrotha, the Gravetide; Omniscience; Quilled Greatwurm; Tinybones, Bauble Burglar; Vizier of the Menagerie |
| Snapshot object-class and subtype predicates | `engine-snapshot-object-classification` | Arahbo, the First Fang; Crossway Troublemakers; Gate Colossus; Gateway Sneak; High-Society Hunter; Infernal Vessel; Kalastria Highborn; Lathliss, Dragon Queen; Midnight Reaper; Spinner of Souls; Valkyrie's Call; Wildborn Preserver; Youthful Valkyrie |
| Snapshot combat-state predicates | `engine-snapshot-combat-state` | Garna, Bloodfist of Keld |
| `StatOf` that can report no information | `engine-statof-reference-channel` | Heartfire Immolator; Halana and Alena, Partners; Ovika, Enigma Goliath; Prime Speaker Zegana; Bite Down; Felling Blow |
| Current-state conditions that don't consume LKI | `engine-condition-lki-context` | Affectionate Indrik |

## Done when

- Every row's mechanism is built, with engine tests for its distinct binding and liveness
  shapes, using direct core fixtures where card authoring is not present.
- Each witness does what its card requires. No abort is removed by turning a legal term into a
  silent no-op; it goes away only because the mechanism behind it exists.
- Partial-target and departed-source tests assert the effects that remain, not only the absence
  of a panic.
- A replay of the faithful core terms for every Foundations card name reaches no abort. Any
  newly exposed abort is routed to the mechanism ticket that owns it before this ticket closes.

## Not in scope

- Completeness of builtin keyword bodies, parser or authoring convenience, and semantic gaps
  that don't abort.

## Pointers

- `docs/decisions/engine.md` (law 6)
- The owning tickets in the table
