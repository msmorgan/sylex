import Semantics.Check
import Semantics.Check.Card
import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions
open Semantics.Definitions.Action (cast)

open Semantics.Templates.NounPhrase (the)
open Semantics.Templates.Predicate (and or andOr not was)
open Semantics.Templates.Location (and or andOr not)
open Semantics.Templates.NounPhrase (and or)
open Semantics.Templates.Timing (not)
open Semantics.Templates.Replacement (doIf)

open Semantics.Templates.Ability (spell)
open Semantics.Templates.Action (doIf target)
open Semantics.Templates.Condition (and not or was)
open Semantics.Templates.DetPhrase (the)
open Semantics.Templates.NounPhrase (andOr)

namespace Semantics.Proofs.DefinitionsAndTemplates


theorem destroyBindsItsSubject (subject : NounPhrase) :
    destroy subject =
      .ownScope (some (.action "destroy"))
        (.move (.firstMention "destroy.subject" (.inCaller subject)) (.zone .battlefield none)
          Semantics.Templates.Location.graveyard .departure []) := rfl

theorem harnessBindsItsSubject (subject : NounPhrase) :
    harness subject =
      .ownScope (some (.action "harness"))
        (doIf (not (is (.firstMention "harness.subject" (.inCaller subject))
          (hasDesignation harnessed)))
          (gainDesignation (.laterMention "harness.subject" (.inCaller subject)) harnessed) none) := rfl

theorem fightBindsOtherAndKeepsTheSource (other : NounPhrase) :
    fight other =
      .ownScope (some (.action "fight" .performs))
        (doIf (and [
          is (.firstMention "fight.other" (.inCaller other)) isCreature,
          is (.firstMention "fight.actor" .actor) isCreature])
          (.simultaneously [
            .dealDamage (.statOf (.stat .power) (.laterMention "fight.actor" .actor))
              (.laterMention "fight.other" (.inCaller other)),
            instruct (.laterMention "fight.other" (.inCaller other))
              (.dealDamage (.statOf (.stat .power) (.laterMention "fight.other" (.inCaller other)))
                (.laterMention "fight.actor" .actor))])
          none) := rfl

theorem okAirbendOne :
    (airbend (target isCreature)).check (DRS.ofList []) = [] := by decide

theorem okAirbendUpToTwo :
    (airbend (.described (.target (upTo 2)) isCreature)).check (DRS.ofList []) = [] := by decide

theorem okConniveOne :
    (connive 1).check (DRS.ofList []) = [] := by decide

theorem okConniveTarget :
    (instruct (target isCreature) (connive 2)).check (DRS.ofList []) = [] := by decide

theorem okRippleFour :
    (ripple 4).check = [] := by decide

private def callerContext : DRS :=
  nextClause (stepAction 64 .postEvent .obj (DRS.ofList []) (tap (target isCreature))).1

theorem keywordCostReadsCaller :
    (cycling (.perform (tap it))).check callerContext = [] := by decide

theorem keywordBodyCannotReadCaller :
    (Ability.keyword (.named "flying") []
      [.static (.cant .any (.block it) {})]).check callerContext =
        [.anaphor .bare .one 0] := by decide

theorem callerPronoun :
    (Action.sequentially [tap (target isCreature), untap it]).check (DRS.ofList []) = [] := by
  decide

theorem definitionCannotReadUnpassedCaller :
    (Action.sequentially [tap (target isCreature),
      .ownScope (some (.action "tap")) (.setStatus .tapped it)]).check (DRS.ofList []) =
        [.anaphor (.atSlot .permanent) .one 0] := by decide

theorem templateKeepsSingleTarget :
    ((stepAction 64 .postEvent .obj (DRS.ofList [])
      (dealDamageOwnPower (target isCreature) (target isCreature))).1.referents.filter
        (fun r ↦ r.det == .target)).length = 2 := by decide

theorem parameterIdentitySurvivesTap :
    (Action.sequentially [tap (target isCreature),
      .setStatus .untapped (.laterMention "tap.subject" .any)]).check (DRS.ofList []) = [] := by
  decide

theorem movedParameterCannotBeReused :
    (Action.sequentially [Semantics.Definitions.Action.exile (target isCreature),
      .setStatus .untapped (.laterMention "exile.subject" .any)]).check (DRS.ofList []) =
        [.bindingIdentity "exile.subject"] := by decide

theorem atomicDefinitionLaw :
    (tap (target isCreature)).check (DRS.ofList []) = [] := by decide

/-- An Action Definition's own Reference Scope header is its one deed: the head gate admits the
action by it, its caused event names it, and stepping it stamps what the body acts on, with no
second deed-carrying node (`Action.ownDeed?`). `cast` writes no other node at all. -/
theorem headerDeedIsReadByTheGateAndTheStepper :
    let spell := cast (target (and [isInstantOrSorcery, .isCard]))
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) spell).1
    spell.eventHeadOk = Bool.true ∧
      spell.namedDeed = some (.action "cast") ∧
      after.referents.map (·.payload.prov.map (·.deeds)) = [some [.action "cast"]] := by decide

/-- No deed is guessed from a body's shape: a bare move to exile, a bare status change and a bare
creation name none. The checker reads a keyword action's deed only from its Definition's
`ownScope` header (`semantics.md` §3.1, "The checker does not guess a deed from a body's
shape"). -/
theorem bareShapesNameNoDeed :
    (Action.move (target isCreature) .anyLocation (some exileZone) .arrival []).namedDeed = none ∧
      (Action.setStatus .tapped (target isCreature)).namedDeed = none ∧
      (Semantics.Definitions.Action.exile (target isCreature)).namedDeed = some (.action "exile") ∧
      (tap (target isCreature)).namedDeed = some (.action "tap") := by decide

theorem revealUpdatesActionStamp :
    (Action.sequentially [tap (target isCreature), Semantics.Definitions.Action.reveal it,
      tap (the (was (Semantics.Definitions.Action.reveal which)))]).check (DRS.ofList []) = [] := by decide

theorem exhaustedTraversalRefuses :
    (stepAction 0 .postEvent .obj (DRS.ofList []) (tap (target isCreature))).2 =
      [.recursionLimit] := by decide

theorem exhaustedWrapperRefuses :
    (stepLocationWith (stepOps 0) (DRS.ofList []) Semantics.Templates.Location.hand).2 =
      [.recursionLimit] := by decide

/-! ## Put and return

"Put" and "return" name no deed: each is ordinary English for a Zone Change [CR#400.7]. Separate
Templates keep the two wordings, and they expand to the same move. -/

theorem putAndReturnExpandToOneTree (subject : NounPhrase) (destination : Location)
    (riders : List MoveRider) (origin : Location) :
    returnToHand subject = putInto subject Semantics.Templates.Location.hand ∧
    returnToHand subject origin = putInto subject Semantics.Templates.Location.hand origin ∧
    returnToBattlefield subject = putOntoBattlefield subject ∧
    returnToBattlefield subject riders origin = putInto subject Semantics.Templates.Location.battlefield origin riders ∧
    returnTo subject destination riders = put subject destination riders :=
  ⟨rfl, rfl, rfl, rfl, rfl⟩

theorem putTakesRiders (s : NounPhrase) (d : Location) (r : List MoveRider) :
    put s d r = putInto s d anywhere r := rfl

/-! ## The bare plural

A universal bare plural and "all" have one rules meaning and one plural referent: "Creatures you
control get +1/+1" and "All creatures you control get +1/+1". Separate Templates keep the two
wordings, and they expand to the same tree. -/

theorem bareAndAllOfExpandToOneTree (p : Predicate) :
    Semantics.Templates.NounPhrase.bare p = allOf p ∧
      getsPt (Semantics.Templates.NounPhrase.bare (and [isCreature, youControl])) (Semantics.Templates.Delta.Amount.up 1) (Semantics.Templates.Delta.Amount.up 1) =
        getsPt (allOf (and [isCreature, youControl])) (Semantics.Templates.Delta.Amount.up 1) (Semantics.Templates.Delta.Amount.up 1) :=
  ⟨rfl, rfl⟩

/-- "Return <subject> from your graveyard to your hand" and "put <subject> into your hand from
your graveyard" name no deed and expand to the same move, its written origin kept [CR#400.7]. -/
theorem graveyardToHandVerbsExpandToOneMove :
    returnToHand (target isCreature) yourGraveyard =
      putInto (target isCreature) Semantics.Templates.Location.hand yourGraveyard := rfl

/-- "X and Y", "X or Y" and "X and/or Y" are three encodings of a noun phrase. -/
theorem nounPhraseCoordinatorsAreDistinct :
    (and [you, target isCreature] : NounPhrase) ≠ (andOr [you, target isCreature] : NounPhrase) ∧
      (or [you, target isCreature] : NounPhrase) ≠ (andOr [you, target isCreature] : NounPhrase) ∧
      (and [you, target isCreature] : NounPhrase) ≠ (or [you, target isCreature] : NounPhrase) := by
  refine ⟨?_, ?_, ?_⟩ <;> intro h <;> cases h

/-- "graveyard, hand, and library", "… or library" and "… and/or library" are three encodings of
a location. -/
theorem locationCoordinatorsAreDistinct :
    (and [yourGraveyard, yourHand, yourLibrary] : Location) ≠
        (andOr [yourGraveyard, yourHand, yourLibrary] : Location) ∧
      (or [yourGraveyard, yourHand, yourLibrary] : Location) ≠
        (andOr [yourGraveyard, yourHand, yourLibrary] : Location) := by
  refine ⟨?_, ?_⟩ <;> intro h <;> cases h

/-- Flying's "with flying and/or reach" is not "with flying or reach" [CR#702.9b]. -/
theorem flyingAndOrReachIsNotFlyingOrReach :
    (andOr [hasKeyword (.the (.named "flying")), hasKeyword (Semantics.KeywordTerm.the reach.label)] : Predicate) ≠
      (or [hasKeyword (.the (.named "flying")), hasKeyword (Semantics.KeywordTerm.the reach.label)] : Predicate) := by
  intro h; cases h

end Semantics.Proofs.DefinitionsAndTemplates
