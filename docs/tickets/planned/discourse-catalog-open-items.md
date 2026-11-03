---
needs: [semantics-rust-rewrite]
---
# Behaviour the Lean proofs record as wrong or unchecked

Found while distilling `docs/semantics-spec-catalog.md` (`lean-proofs-spec-catalog`). The Lean
sources pin or document these as gaps; the catalog does not carry them as spec. `discourse` (or
`semantics`) settles each.

1. **A battlefield location without "card" reads an object no longer there.**
   `LinkedReadback.gapBattlefieldLocationIsNotAZone` accepts "the exiled thing in the battlefield"
   though the card is in exile; with `isCard` written the same restriction is refused
   (`LinkedReadback.badLinkedObjectInAnotherZone`). Refuse both.
2. **Who pays a cost is not checked.** `Actor.badHandedOffLifeCost`,
   `Actor.badNestedHandoffInnermost` and `Actor.badGrantedAbilityKeepsItsOwnPerformer` say so;
   `lean-checker-sweep-costs` closed without it. Decide whether `discourse` checks a cost's payer,
   and pin it either way.
3. **That only a rule confers the commander designation is not asserted**
   (`Static.badGainsCommander`; `lean-checker-sweep-game-rules` closed without it). Decide whether
   an action granting it is refused.
4. **An empty coordination is refused under two kinds.** Predicate `.or []` and Static
   `.conjunction none []` give `.nonEmpty`; NounPhrase `.and []` / `.or []` and Cost `.or []` give
   `.atLeastTwo` (`Zone.badEmptyOr`, `Composition.badEmpty*`). Pick one, or record why they differ.
5. **Entries marked FLAG DEFECT** in the catalog may encode
   `discourse-parameter-argument-resolution`'s defect, notably the refusal counts of
   `CallerScope.expressionAmountObservesNewOutcomes` and
   `CallerScope.nestedTemplatesRereadTheCurrentOutcome` and the target count of
   `DefinitionsAndTemplates.templateKeepsSingleTarget`. Re-derive them after that fix rather than
   porting them as written.
6. **Fragments from cards that may be out of scope.** The catalog's fragment entries flag Gabriel
   Angelfire, Celebr-8000, Boommobile, Atomwheel Acrobats, Contractual Safeguard, Frontline Heroism
   and Don't Blink as possibly Alchemy or Un-set. Drop the Alchemy ones (not Vintage-legal); keep
   Un-set ones deferred.
7. **A coordinated trigger subject loses its anaphor.** *Pine Walker*: "Whenever this creature or
   another creature you control is turned face up, untap that creature." With the two subjects
   written as a coordination, "that creature" does not resolve, so the canon card is spelled with
   the equivalent "a creature you control" (`idris-retire-cards-idr`). `discourse` resolves "that
   creature" to whichever coordinated subject the event matched; respell the card then.
8. **Lean's line context reaches a sacrificed land's successor.** `lean-discourse-export` found that
   Lean's `context` pass (`lineTextContextWith`) steps a later sequence step before earlier ones
   on *Flooded Strand* and *Afflicted Deserter // Werewolf Ransacker*: the pronoun or demonstrative
   there resolves to the sacrificed land's (or artifact's) moved successor, while the `check` pass
   correctly resolves *Flooded Strand*'s "it" to the searched card. The context a line leaves for
   later lines is stepped in order in `discourse`; the export's readings whose `lastIn` holds only
   `context` are not expected to match.
