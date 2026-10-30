import SylFixtures
import Semantics.Check
import Semantics.Check.Card
import Semantics.Definitions
import Semantics.Check.NumberSlots
import Semantics.Check.LocalShape
import Semantics.Proofs.Helpers

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (a counted)
open Semantics.Templates.Predicate (did)
open Semantics.Templates.StaticSpec (replace)
open Semantics.Templates.Replacement (doIf)

open Semantics.Templates.Ability (spell)
open Semantics.Templates.Action (doIf replace target)
open Semantics.Templates.ColorTerm (chosen)
open Semantics.Templates.Condition (did)
open Semantics.Templates.DetPhrase (a)
open Semantics.Templates.SliceCount (counted)

namespace Semantics.Proofs.CallerScope

export SylFixture.Semantics.Templates (drawTwice forwardedSubject subjectPower repeatedValueSum
  literalEcho quantityFor graveyardFor creatureCondition powerEquals lifeCost powerBoost
  unusedSubject secondSubject moveSubject rereadDamage definedPower eventWithSubject
  playerIgnoringSubject nestedRereadDamage)

private def selected (drs : DRS) (body : NounPhrase) : List Referent :=
  let result := stepNounPhrase 64 drs .other false body
  result.2.1.filterMap result.1.findById?
private def callerCreature : DRS := DRS.ofList
  [{ det := .the, plur := .one, payload := .object [.creature] (some .battlefield) none none none }]
private def callerCreatureSpell : DRS := DRS.ofList
  [{ det := .the, plur := .one, payload := .object [.creature] (some .stack) none none none }]

private def selectThenTap (subject : NounPhrase) : Action :=
  .ownScope none (.sequentially
    [.setStatus .tapped (.firstMention "select.subject" (.inCaller subject)),
      .setStatus .untapped (.laterMention "select.subject" .any)])
private def selectThenMove (subject : NounPhrase) : Action :=
  .ownScope none (.move (.firstMention "select.subject" (.inCaller subject))
    .anyLocation (some graveyard) .arrival [])
private def announceThen (chosen : NounPhrase) (body : Action) : Action :=
  .ownScope none (.sequentially [.setStatus .tapped (.firstMention "announce.subject" (.inCaller chosen)),
    .inCaller body])

/-- "Draw a card. If you control three or more artifacts, <replacement> instead", the
replacement supplied by the caller. -/
private def drawUnlessReplaced (replacement : Replacement) : Action :=
  .ownScope none (.replace (draw 1)
    (doIf (compareAmt (countOf isArtifact) .atLeast 3) (.inCaller replacement)))
/-- The same Definition, forgetting that its replacement is its caller's. -/
private def drawUnlessReplacedHere (replacement : Replacement) : Action :=
  .ownScope none (.replace (draw 1)
    (doIf (compareAmt (countOf isArtifact) .atLeast 3) replacement))

private def explicitExpansion : Card := .singleFaced
  { characteristics := {
      name := "Expansion witness", types := [.sorcery],
      text := [spell (drawTwice 1)] } }
theorem explicitExpansionChecksItsResult : explicitExpansion.check = [] := by decide

theorem templateArgumentsAreEvaluatedAtTheirTextualUse :
    (unusedSubject it).check (DRS.ofList []) = [] ∧
      (moveSubject (secondSubject (target isArtifact) it)).check (DRS.ofList []) = [.anaphor .bare .one 0] := by decide
theorem unusedSubjectDoesNotIntroduceAReferent :
    (actionResult (DRS.ofList []) (unusedSubject (target isCreature))).referents.countP
      (fun ref => ref.det == .target) = 0 := by decide
theorem identicalArgumentsRemainSeparateSelections :
    countOnes .object (actionResult (DRS.ofList [])
      (instruct (target isCreature) (fight (target isCreature)))) = 2 := by decide

theorem namedParameterIsSelectedOnce :
    let after := actionResult (DRS.ofList []) (selectThenTap (target isCreature))
    countOnes .object after = 1 ∧
      (identityBinding "select.subject" after).map (·.id) = some 0 := by decide
theorem nestedTemplatePreservesParameterIdentity :
    let drs := nounResult (DRS.ofList []) (.firstMention "subject" (target isCreature))
    (selected drs (forwardedSubject (.laterMention "subject" .any))).map (·.id) = [0] := by decide
theorem namedParameterCannotReadAnAmountSlot :
    NounPhrase.check (DRS.ofList [letterB .x]) (.laterMention "subject" .any) =
      [.bindingIdentity "subject"] := by decide
theorem movedParameterLosesItsIdentity :
    let after := actionResult (DRS.ofList []) (selectThenMove (target isCreature))
    identityBinding "select.subject" after = none ∧
      (after.visibleReferents.filter (fun r => r.zone == some .graveyard)).length = 1 ∧
      after.findById? 0 = none := by decide

theorem suppliedBodyKeepsItsCallerPronoun :
    (announceThen (target isArtifact) (putInto it graveyard)).check callerCreature = [] := by decide
theorem suppliedBodyUpdatesItsOriginalReferent :
    let after := actionResult callerCreature (announceThen (target isArtifact) (putInto it graveyard))
    (after.visibleReferents.filter (fun r => r.ty == [.creature])).map (·.zone) = [some .graveyard] ∧
      (after.visibleReferents.filter (fun r => r.ty == [.artifact])).map (·.zone) = [some .battlefield] := by decide

theorem forgottenParameterDoesNotSurvivePrivately :
    let drs := nounResult (DRS.ofList [])
      (.firstMention "library.subject" (target (.and [.isCard, .in_ yourLibrary])))
    let after := actionResult drs shuffle
    NounPhrase.check after (.laterMention "library.subject" .any) = [.bindingIdentity "library.subject"] ∧
      countReach .bare .one after = 0 := by decide

theorem quantitiesAcceptTemplateValues : (stepQuantityWith (stepOps 64) (DRS.ofList []) (quantityFor 1)).2 = [] := by decide
theorem templateValuesPreserveLiteralFacts :
    let value := literalEcho 1
    (value.exact, value.plur, value.unit) =
      (some 1, .one, true) := by decide
theorem amountTemplatesCanReuseAValue :
    (repeatedValueSum .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) = [] := by decide
theorem quantityParameterPreservesOneCardinality :
    ((quantityFor 1).plur) = .one := by decide
theorem zonesAcceptTemplateOwners : (graveyardFor you).check (DRS.ofList []) = [] := by decide
theorem costsAcceptTemplateValues : (lifeCost 1).check (DRS.ofList []) = [] := by decide
theorem staticSpecsAcceptTemplateValues :
    (powerBoost (target isCreature) .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) = [] := by decide
theorem amountTemplatesAcceptSubjects : (subjectPower (target isCreature)).check (DRS.ofList []) = [] := by decide
theorem amountTemplatesIntroduceSelectionOnce :
    countOnes .object ((stepAmount 64 (DRS.ofList []) (subjectPower (target isCreature))).1) = 1 := by decide
theorem amountTemplatesRejectMissingSubjects :
    (subjectPower it).check (DRS.ofList []) = [.anaphor .bare .one 0] := by decide

theorem predicateTemplatesAcceptValues :
    Predicate.checkAt .object (DRS.ofList [outcomeB .damageDealt]) (powerEquals .thatMuch) = [] := by decide

theorem conditionsPublishSubjectOnce :
    countOnes .object ((stepCondition 64 (DRS.ofList []) (creatureCondition (target isCreature))).1) = 1 := by decide
theorem conditionsRefineOriginalIdentity :
    let before := DRS.ofList [{
      det := .the, plur := .one,
      payload := .object [] (some .battlefield) none none none}]
    let after := (stepCondition 64 before (creatureCondition it)).1
    (after.findById? 0).map (·.ty) = some [.creature] ∧ after.nextId = before.nextId := by decide

theorem resolvedSubjectRetainsIdentityAndChangesZone :
    let refs := selected callerCreatureSpell (forwardedSubject (.becomes it))
    refs.map (·.id) = [0] ∧ refs.map (·.zone) = [some .battlefield] := by decide
theorem amountReadsPreserveNounContextUpdates :
    (((stepAmount 64 callerCreatureSpell (subjectPower (forwardedSubject (.becomes it)))).1).findById? 0).bind
      (·.zone) = some .battlefield := by decide
theorem nounCoordinationPreservesContextUpdates :
    ((nounResult callerCreatureSpell (.and [forwardedSubject (.becomes it), .you])).findById? 0).bind
      (·.zone) = some .battlefield := by decide

theorem eventParametersPublishTheirOutcome :
    countOutcomes .damageDealt (GameEvent.extend (DRS.ofList [])
      (.does (.actor thisCreature) (.dealDamage .anyAmount .you))) = 1 := by decide
theorem templateUseRemainsInsideCondition :
    Action.doIf (.compareAmt 1 .eq 1) (moveSubject (target isCreature)) none =
      .doIf (.compareAmt 1 .eq 1)
        (.move (target isCreature) .anyLocation (some graveyard) .arrival []) none := by rfl
theorem templateUseRemainsInsideRepetition :
    Action.repeat_ (.fixed 2 (moveSubject (target isCreature))) =
      .repeat_ (.fixed 2
        (.move (target isCreature) .anyLocation (some graveyard) .arrival [])) := by rfl
theorem parameterizedGroupsRemainCountable :
    (Amount.countOf (forwardedSubject (counted (exactly 2) isCreature))).check (DRS.ofList []) = [] := by decide
theorem forwardedYouRemainsThePayer : (forwardedSubject you).isYou = true := by decide
theorem forwardedOpponentIsNotThePayer : (forwardedSubject anOpponent).isYou = false := by decide

theorem firstMentionUsesTextualOrder :
    Action.check (DRS.ofList []) (.sequentially
      [.setStatus .tapped (.laterMention "late" .any),
        .setStatus .untapped (.firstMention "late" (target isCreature))]) = [.bindingIdentity "late"] := by decide
theorem firstMentionMakesLaterMentionAvailable :
    Action.check (DRS.ofList []) (.sequentially
      [.setStatus .tapped (.firstMention "early" (target isCreature)),
        .setStatus .untapped (.laterMention "early" .any)]) = [] := by decide

theorem scopedNumberRegimeIsPreserved :
    (Action.ownScope none (.establish (.letterDefinition .x 2) none)).numberSlots =
      [(.lit 2, .signed)] := rfl

theorem possessorQueryPreservesSubjectUpdate :
    ((nounResult callerCreatureSpell (.possessorOf .controller (.becomes it))).findById? 0).bind
      (·.zone) = some .battlefield := by decide

theorem sameSelectedSubjectIsReadBackThroughTemplate :
    (Action.exchange (.values (subjectPower (target isCreature)) (powerOf it))).check (DRS.ofList []) = [] := by decide
theorem independentValueSubjectsRemainIndependent :
    (Action.exchange (.values (subjectPower (target isCreature))
      (subjectPower (target isCreature)))).check (DRS.ofList []) = [] := by decide
theorem expressionAmountObservesNewOutcomes :
    (rereadDamage .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) = [.quantOutcomeInScope 2] := by decide

theorem scopedStaticDefinitionsRemainVisibleToPrintedBoxes :
    (definedPower thisCreature).definedStats = [.power] := by decide

theorem independentCounterpartDoesNotBecomeSelfByPayloadEquality :
    StaticSpec.check (DRS.ofList [])
      (.must (.actor (target isCreature)) (.block (target isCreature))
        {}) = [] := by decide

theorem containingNounRetainsIndependentSelection :
    let after := nounResult (DRS.ofList []) (.possessorOf .controller (forwardedSubject (target isCreature)))
    (after.visibleReferents.filter (fun ref => ref.ty == [.creature])).length = 1 ∧
      countOnes .player after = 1 ∧
      after.visibleReferents.map (fun ref => (ref.id, ref.zone)) =
        [(1, none), (0, some .battlefield)] := by decide

theorem secondSubjectSelectsOnlyItsTextuallyUsedArgument :
    let after := actionResult (DRS.ofList []) (moveSubject (secondSubject (target isArtifact) (target isCreature)))
    after.visibleReferents.map (·.ty) = [[.creature]] ∧
      after.visibleReferents.map (·.zone) = [some .graveyard] := by decide

theorem nestedForwardingPreservesMoveAndTapBehavior :
    Action.check (DRS.ofList []) (.sequentially
      [moveSubject (forwardedSubject (target isCreature)), .setStatus .tapped (that .permanent)]) =
        [.anaphor (.word .permanent) .one 0] ∧
      Action.check (DRS.ofList []) (.sequentially
        [.setStatus .tapped (forwardedSubject (target isCreature)), .setStatus .untapped (that .permanent)]) = [] := by decide

theorem sourceViewPreservesStaticDefinitionRole :
    StaticSpec.check (DRS.ofList []) (definedPower (forwardedSubject thisCreature)) = [] := by decide

theorem forwardedBareThisRetainsDefinitionZoneExemption :
    (destroy (forwardedSubject .this)).check (DRS.ofList []) = [] := by decide

theorem forwardedLibraryOwnerRetainsOpponentClassification :
    (Action.ownScope (some (.action "fateseal")) (lookAndSortSlice (.slice .top 2 (libraryOf (forwardedSubject anOpponent))) onBottom
        [(.inOrder .anyOrder)])).check
      (DRS.ofList []) = [] := by decide

theorem librarySlicePreservesExplicitOwnerUpdate :
    ((nounResult callerCreatureSpell
      (.slice .top 2 (libraryOf (.possessorOf .controller (.becomes it))))).findById? 0).bind
        (·.zone) = some .battlefield := by decide

theorem forwardedCoordinationRetainsTwoParties :
    (Action.exchange (.lifeTotals (forwardedSubject (.and [.you, anOpponent])))).check
      (DRS.ofList []) = [] := by decide



theorem eventTemplateAcceptsItsSubject :
    (stepGameEvent 64 (DRS.ofList []) (eventWithSubject (forwardedSubject thisCreature))).2 =
      [] := by decide

theorem eventTemplatePublishesItsOutcome :
    countOutcomes .damageDealt
      (stepGameEvent 64 (DRS.ofList []) (eventWithSubject (forwardedSubject thisCreature))).1 =
      1 := by decide

theorem forwardedSourceRetainsItsSelfDefinitionRole :
    (forwardedSubject thisCreature).selfDefinedOk = true := by decide

theorem callerExitDoesNotResurrectForgottenMentions :
    let caller := nounResult (DRS.ofList [])
      (.firstMention "library.subject" (a (.and [.isCard, .in_ yourLibrary])))
    let inside := enterOwnScope caller
    let restored := leaveCaller inside (afterShuffle (enterCaller inside))
    restored.referents.map (fun ref => (ref.id, ref.kind, ref.zone)) =
      [(0, .player, none)] ∧ identityBinding "library.subject" restored = none ∧
      countReach .bare .one restored = 0 := by decide

theorem possessorQueryPreservesItsWholeContext :
    let after := nounResult callerCreatureSpell
      (.possessorOf .controller (forwardedSubject (.becomes it)))
    after.visibleReferents.map (fun ref => (ref.id, ref.kind, ref.zone)) =
      [(1, .player, none), (0, .object, some .battlefield)] := by decide

/-- The coordination leaves its conjuncts' referents as they were, and publishes their group. -/
theorem nounCoordinationPreservesItsWholeContext :
    let after := nounResult callerCreatureSpell
      (.and [forwardedSubject (.becomes it), .you])
    (after.visibleReferents.filter (·.id < 2)).map (fun ref => (ref.id, ref.kind, ref.zone, ref.partOf)) =
      [(1, .player, none, some 2), (0, .object, some .battlefield, none)] ∧
      (after.findById? 2).map (fun ref => (ref.plur, ref.kind)) =
        some (.many, .join .object .player) := by decide

theorem unusedLibraryOwnerArgumentDoesNotMutateTheCaller :
    let after := nounResult callerCreatureSpell
      (.slice .top 2 (libraryOf (playerIgnoringSubject (.becomes it))))
    after.visibleReferents.map (fun ref => (ref.id, ref.kind, ref.zone)) =
      [(2, .object, some .library), (1, .player, none), (0, .object, some .stack)] := by decide

theorem templatePatientCannotReadItsCoargument :
    Action.check (DRS.ofList []) (.establish
      (.must (.actor (target isCreature)) (.block (forwardedSubject it)) {})
      (some thisTurn)) = [.anaphor .bare .one 0] := by decide

theorem nestedTemplatesRereadTheCurrentOutcome :
    (nestedRereadDamage .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) =
      [.quantOutcomeInScope 2, .quantOutcomeInScope 3, .quantOutcomeInScope 4] := by decide

theorem amountTemplateLetterReadDoesNotIntroduceItsSourceTwice :
    let amount := literalEcho (.letter .x)
    let first := (stepAmount 64 (DRS.ofList []) amount).1
    let second := (stepAmount 64 first amount).1
    second.referents = first.referents ∧ second.nextId = first.nextId ∧
      countLetter .x second = 1 := by decide

theorem unusedSubjectDoesNotRequireABodyUse (subject : NounPhrase) :
    (unusedSubject subject).check (DRS.ofList []) = [] := by rfl

theorem returnedSecondTemplateSubjectChecks :
    (moveSubject (secondSubject (target isCreature) (target isCreature))).check (DRS.ofList []) =
      [] := by decide

theorem targetGroupsRemainCountableThroughTemplate :
    (Amount.countOf (forwardedSubject (.described (.target (exactly 2)) isCreature))).check
      (DRS.ofList []) = [] := by decide

theorem templateBareThisRetainsScopedZoneExemption :
    (Action.ownScope (some (.action "destroy")) (moveSubject .this)).check (DRS.ofList []) =
      [] := by decide

/-- "Draw a card. If it's a creature, tap it instead.": the replacement's action is a new clause
after its condition, so both can name the same creature. -/
theorem conditionalReplacementActionFollowsItsCondition :
    Action.check callerCreature
      (replace (draw 1) (doIf (is it isCreature) (instead (tap it)))) = [] := by
  decide

/-- A Definition's condition over its caller's replacement reads the caller's action in the
caller's scope: "it" names the caller's creature. -/
theorem conditionOverCallerReplacementReadsItsActionInTheCallersScope :
    Action.check callerCreature (drawUnlessReplaced (instead (tap it))) = [] := by
  decide

/-- It keeps the caller's own condition inside its own, read in the caller's scope too. -/
theorem conditionOverCallerReplacementReadsItsConditionInTheCallersScope :
    Action.check callerCreature
      (drawUnlessReplaced (doIf (is it isCreature) (instead (draw 2)))) = [] := by
  decide

/-- "If you sacrificed a creature this turn": a caller's condition introducing the creature its
action reads. -/
private def sacrificedACreature : Condition :=
  did (Semantics.Subject.actor you) (sacrifice (a isCreature)) (.duration soFarThisTurn none)

/-- "… If you sacrificed a creature this turn, return it to its owner's hand instead.", the
replacement the caller's: its condition and action are read in one caller frame, the action a
new clause after the condition, so "it" names the creature the condition introduced. -/
theorem callerConditionIntroducesWhatItsActionReads :
    Action.check (DRS.ofList [])
      (drawUnlessReplaced
        (doIf sacrificedACreature (instead (returnToHand it graveyard)))) = [] := by
  decide

/-- Read in two caller frames instead, one for the condition and one for the action, the
creature the condition introduced is gone by the time the action reads "it". -/
theorem callerConditionReadInItsOwnFrameLosesItsReferent :
    Action.check (DRS.ofList [])
      (.ownScope none (.replace (draw 1)
        (.onlyIf
          (.and [compareAmt (countOf isArtifact) .atLeast 3, .inCaller sacrificedACreature])
          (.instead (.inCaller (returnToHand it graveyard)))))) = [.anaphor (.atSlot .card) .one 0] := by
  decide

/-- Read in the Definition's own scope instead, the caller's action names nothing. -/
theorem callerReplacementActionReadHereNamesNothing :
    Action.check callerCreature (drawUnlessReplacedHere (instead (tap it))) =
      [.anaphor (.atSlot .permanent) .one 0] := by
  decide

/-- Nor does the caller's condition. -/
theorem callerReplacementConditionReadHereNamesNothing :
    Action.check callerCreature
      (drawUnlessReplacedHere (doIf (is it isCreature) (instead (draw 2)))) =
      [.anaphor .bare .one 0] := by
  decide

end Semantics.Proofs.CallerScope
