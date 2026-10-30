import Generated.Canon
import SylFixtures
import NormalFixtures
import LegacyFixtures
import SylGen.Emit
import Semantics.CanonCheck

example : @SylFixture.Semantics.Templates.Action.doForEachKind =
    @Semantics.Templates.doForEachKind := rfl
example : SylFixture.Semantics.Templates.actionSearch = Semantics.Templates.actionSearch := rfl
example : SylFixture.Semantics.Templates.actionPay = Semantics.Templates.actionPay := rfl

example : NormalFixture.Semantics.Templates.declaredWhite = Semantics.Color.blue := rfl
example : NormalFixture.Semantics.Templates.constructorWhite = Semantics.Color.white := rfl
example : NormalFixture.Semantics.Templates.optionalWhite = some Semantics.Color.blue := rfl
example : NormalFixture.Semantics.Templates.declaredSearch = Semantics.Action.sequentially [] := rfl
example (scope : Semantics.Location) (for_ : Semantics.NounPhrase) :
    NormalFixture.Semantics.Templates.actionSearch for_ scope =
      Semantics.Action.search scope for_ := rfl
example (scope : Semantics.Location) (for_ : Semantics.NounPhrase) :
    NormalFixture.Semantics.Templates.constructorSearch scope for_ =
      Semantics.Action.search scope for_ := rfl
example : SylFixture.Semantics.Definitions.aegisCounter =
    Semantics.Definitions.aegisCounter := rfl
example : SylFixture.Semantics.Definitions.aegisCounter_definition =
    Semantics.Definitions.aegisCounter_definition := rfl
example : SylFixture.Semantics.Definitions.haunting = Semantics.Definitions.haunting := rfl
example : SylFixture.Semantics.Definitions.bear = Semantics.Definitions.bear := rfl
example : SylFixture.Semantics.Templates.yourHand = Semantics.Templates.yourHand := rfl
example : SylFixture.Semantics.Templates.libraryOf = Semantics.Templates.libraryOf := rfl
example : SylFixture.Semantics.Templates.thisCreature = Semantics.Templates.thisCreature := rfl
example : SylFixture.Semantics.Definitions.fight = Semantics.Definitions.fight := rfl
example : SylFixture.Semantics.Definitions.flying = Semantics.Definitions.flying := rfl

open Semantics in
example (who : NounPhrase) :
    SylFixture.Semantics.Templates.repeatedNoun who =
      Templates.Condition.and [
        Templates.is (.firstMention "repeatedNoun.who" who) Templates.isCreature,
        Templates.is (.laterMention "repeatedNoun.who" who) Templates.isCreature] := rfl

open Semantics in
example (who : NounPhrase) :
    SylFixture.Semantics.Templates.alternatives who =
      Templates.Predicate.or [
        Templates.controlledBy (.firstMention "alternatives.who" who),
        Templates.controlledBy (.firstMention "alternatives.who" who)] := rfl

open Semantics in
example (who : NounPhrase) :
    SylFixture.Semantics.Templates.eachKind who =
      Action.doForEachKind .color (some (.firstMention "eachKind.who" who)) .color
        (Templates.instruct (.laterMention "eachKind.who" who) (Templates.draw 1)) := rfl

example : SylFixture.Semantics.Templates.omittedArgument = (-1 : Int) := rfl
example : SylFixture.Semantics.Templates.presentAtom =
    some SylFixture.Semantics.Templates.thisCreature := rfl
example : SylFixture.Semantics.Templates.optionalList =
    some [SylFixture.Semantics.Definitions.flying] := rfl

example : SylFixture.Semantics.Cards.Forest = Semantics.Cards.Forest := rfl
example : SylFixture.Semantics.Definitions.threshold = Semantics.Definitions.threshold := rfl
example : SylFixture.Semantics.Definitions.citysBlessing =
    Semantics.Definitions.citysBlessing := rfl
example : SylFixture.Semantics.Definitions.citysBlessing_definition =
    Semantics.Definitions.citysBlessing_definition := rfl
example : SylFixture.Semantics.Templates.dispatchNatural = (5 : Nat) := rfl
example : SylFixture.Semantics.Templates.dispatchInteger = (-5 : Int) := rfl
example : SylFixture.Semantics.Templates.exactOptionalUse = some (-2 : Int) := rfl
example : SylFixture.Semantics.Templates.innerOptionalUse = some (-4 : Int) := rfl
example : SylFixture.Semantics.Templates.defaultsInOrderUse = (-1 : Int) := rfl

example : SylFixture.Semantics.Cards.«Branchloft Pathway // Boulderloft Pathway» = Semantics.Cards.«Branchloft Pathway // Boulderloft Pathway» := rfl

example : SylFixture.Semantics.Cards.«Merfolk Secretkeeper // Venture Deeper» = Semantics.Cards.«Merfolk Secretkeeper // Venture Deeper» := rfl

example : SylFixture.Semantics.Cards.«Blitz Automaton» = Semantics.Cards.«Blitz Automaton» := rfl

example : SylFixture.Semantics.Cards.«Student of Warfare» = Semantics.Cards.«Student of Warfare» := rfl

example : SylFixture.Semantics.Cards.«Profit // Loss» = Semantics.Cards.«Profit // Loss» := rfl

example : SylFixture.Semantics.Cards.«Glassworks // Shattered Yard» = Semantics.Cards.«Glassworks // Shattered Yard» := rfl

example : SylFixture.Semantics.Cards.«Neglected Heirloom // Ashmouth Blade» = Semantics.Cards.«Neglected Heirloom // Ashmouth Blade» := rfl

example : SylFixture.Semantics.Cards.«Bushi Tenderfoot // Kenzo the Hardhearted» = Semantics.Cards.«Bushi Tenderfoot // Kenzo the Hardhearted» := rfl

example : SylFixture.Semantics.Definitions.fight.deed = Semantics.Definitions.fight.deed := rfl
example : SylFixture.Semantics.Definitions.flying.label = Semantics.Definitions.flying.label := rfl
example : SylFixture.Semantics.Definitions.bear_definition = Semantics.Definitions.bear_definition := rfl
example : SylFixture.Semantics.Definitions.haunting_definition = Semantics.Definitions.haunting_definition := rfl

example : NormalFixture.Semantics.Templates.Int.shared = (-1 : Int) := rfl
example : NormalFixture.Semantics.Templates.Bool.shared = true := rfl
example : NormalFixture.Semantics.Templates.this_ = (1 : Int) := rfl
example : NormalFixture.Semantics.Templates.useThis = (1 : Int) := rfl
example : NormalFixture.Semantics.Templates.named = (2 : Int) := rfl
example : NormalFixture.Semantics.Templates.words (this_ := 1) (in_ := 2) (from_ := 3) (by_ := 4)
    (or_ := 5) (exists_ := 6) (where_ := 7) («variable» := 8) = [1,2,3,4,5,6,7,8] := rfl
example : NormalFixture.Semantics.Definitions.withoutDeed =
    Semantics.Action.ownScope none (.sequentially []) := rfl
example : NormalFixture.Semantics.Definitions.wordDeed.deed =
    Semantics.Deed.action "customWord" := rfl

open Semantics in
example (who : NounPhrase) : SylFixture.Semantics.Templates.recordOrder who =
    { subj := .actor (.firstMention "recordOrder.who" who),
      action := Templates.instruct (.laterMention "recordOrder.who" who) (Templates.draw 1),
      within := none } := rfl
open Semantics in
example (ability : Ability) : SylFixture.Semantics.Definitions.alreadyWord ability =
    .italicHead (.abilityWord (.named "inner")) ability := rfl
open Semantics in
example (who : NounPhrase) : SylFixture.Semantics.Definitions.repeatedAbilityWord who =
    Templates.abilityWord (.named "repeated") (Templates.Ability.spell
      (Templates.simultaneously [
        Templates.instruct who (Templates.draw 1),
        Templates.instruct who (Templates.draw 1)])) := rfl

open Lean in
run_cmd do
  let env ← getEnv
  let entries := Semantics.Registry.expectedTypeDispatchExt.getState env
  unless ((entries.find? `fight).getD []).any
      (·.declaration == `SylFixture.Semantics.Definitions.fight) do
    throwError "generated declaration missing from dispatch registry"
  unless (Semantics.Registry.canonCardsExt.getState env).any
      (·.declaration == `SylFixture.Semantics.Cards.Forest) do
    throwError "generated card missing from canon registry"
  for name in [`SylFixture.Semantics.Definitions.flying.term,
      `NormalFixture.Semantics.Definitions.withoutDeed.deed,
      `NormalFixture.Semantics.Definitions.abilityDeed.deed] do
    if env.contains name then throwError "unexpected generated constant {name}"
  unless ((entries.find? `in).getD []).isEmpty do
    throwError "unexpected surface-spelling dispatch entry"
  unless ((entries.find? `in_).getD []).any
      (·.declaration == `NormalFixture.Semantics.Templates.in_) do
    throwError "missing Lean-spelling dispatch entry"

example : (match SylGen.parse "header.syl" "# File header\n\ntemplate x: Int { body = 1 }" with
    | .ok ds => ds[0]!.documentation.isEmpty
    | .error _ => false) = true := by native_decide

private def conflictingDeed : Semantics.Action :=
  .ownScope (some (.action "fight" .none)) (.sequentially [])
open Lean Elab Command in
run_cmd do
  let saved ← get
  let mut rejected := false
  try
    elabCommand (← `(syl_register "action" conflictingDeed ""))
  catch error =>
    rejected := (← error.toMessageData.toString).contains "must agree on DeedPermanent"
  for message in (← get).messages.toList do
    if (← message.toString).contains "must agree on DeedPermanent" then rejected := true
  set saved
  unless rejected do throwError "conflicting action deed was accepted"

example : NormalFixture.Semantics.Definitions.outerWord =
    NormalFixture.Semantics.Templates.innerWord := rfl

example : (match SylGen.parse "docs.syl"
    "primitive lit(value: Int): Amount\n# Attached documentation\ntemplate x: Int { body = 1 }" with
    | .ok ds => ds[1]!.documentation == " Attached documentation\n"
    | .error _ => false) = true := by native_decide

example : NormalFixture.Semantics.Definitions.wrappedWord =
    NormalFixture.Semantics.Templates.abilityWord (.named "fresh")
      (.spell none (.sequentially [])) := rfl

open Lean Meta Elab Command in
run_cmd do
  let some value := (← getConstInfo `SylFixture.Semantics.Definitions.threshold).value?
    | throwError "missing ability word"
  liftTermElabM <| lambdaTelescope value fun _ body => do
    unless body.getAppFn.isConstOf `Semantics.Templates.abilityWord do
      throwError "ability word did not retain its Template application"

example : NormalFixture.Semantics.Templates.or (value := 5) = (5 : Int) := rfl

example : NormalFixture.Semantics.Definitions.calledDeed.deed =
    Semantics.Deed.action "customWord" := rfl

example : NormalFixture.Semantics.Templates.naturalInteger 3 = (3 : Int) := rfl
example : NormalFixture.Semantics.Templates.colorSelection = Semantics.ColorSpec.some [.blue] := rfl
example : NormalFixture.Semantics.Templates.callsWithLocalName .you = Semantics.Subject.actor .you := rfl
example : NormalFixture.Semantics.Templates.multiline = "first\nsecond" := rfl

namespace LegacyBundlePins
open Semantics Semantics.Templates
open LegacyFixture.Semantics.Definitions LegacyFixture.Semantics.Templates
example : LegacyFixture.Semantics.Cards.«Bundle fixture».check = [] := by decide
example : bundleQuality = bundleAtom := rfl
example : Action.bundleShared = Semantics.Action.ownScope (some (.action "bundleShared" .none)) bundleDraw := rfl
example : Ability.bundleShared = .keyword (.named "bundleShared") [] [Semantics.Templates.Ability.spell Action.bundleShared] := rfl
example : aQuality = Semantics.Predicate.and [Semantics.Predicate.isToken, Semantics.Predicate.isCard] := rfl
example : aSubject = Semantics.NounPhrase.you := rfl
example (subject : NounPhrase) : Action.bundleDevour subject =
    Semantics.Action.ownScope (some (.ofAbility (.named "bundle devour")))
      (Semantics.Definitions.sacrifice (.firstMention "bundleDevour.subject" (.inCaller subject))) := rfl
example : Ability.bundleDevour = Semantics.Ability.keyword (.named "bundle devour") []
    [Semantics.Templates.Ability.spell (Action.bundleDevour thisCreature)] := rfl
theorem registryConferralMayReferToItsCatalogValue : bundleBearer_definition = .subtype bundleBearer [bundleConferral] := rfl
theorem appliedReturnTypeInBundle : bundlePredicates = [hasSubtype bundleBearer] := rfl
end LegacyBundlePins

namespace GeneratedCanonPins
open Semantics
open LegacyFixture.Semantics.Cards
noncomputable def checkWitness : Card := «Canon Witness»
syl_register "card" checkWitness "checkWitness.syl"
check_canon_card "checkWitness.syl"
example : Card.check checkWitness = [] := ok_checkWitness
theorem canonCardIsCheckedByItsCheckModule :
    Card.check «Canon Witness» = [] := «ok_Canon Witness»
example : Registry.cardFileName "Canon Witness: Split" = "Canon Witness{colon} Split.syl" := by native_decide
end GeneratedCanonPins

namespace DispatchContract
open Semantics NormalFixture.Semantics.Templates NormalFixture.Semantics.Definitions

theorem parameterShadowsEnum : parameterShadowsWord .red = .red := rfl
theorem parameterWrapsOption : optionalParameter .red = some .red := rfl
theorem parameterKeepsExactOption : exactOptionalParameter none = none := rfl
theorem closedEnumWords : enumWords = [.legendary, .basic] := rfl
theorem optionalClosedEnum : optionalEnum = some .upkeep := rfl
theorem directManaAtoms : manaAtoms = [.simple (.generic 2), .simple (.specific (.of .blue))] := rfl
theorem optionalNumericAtom : optionalAmount = some (.lit 3) := rfl
theorem abilityAtLabel : definitionLabel = sourceAbility.label := rfl
theorem abilityAtTerm : definitionTerm = .the sourceAbility.label := rfl
theorem actionAtDeed : actionDeed = calledDeed.deed := rfl
theorem definitionWrapsCaller (subject : NounPhrase) : callerWrapped subject =
    .ownScope (some (.action "callerWrapped"))
      (.setStatus .tapped (.firstMention "callerWrapped.subject" (.inCaller subject))) := rfl
theorem templateRepeatsSelection (subject : NounPhrase) : repeatedSelection subject =
    .sequentially [
      .setStatus .tapped (.firstMention "repeatedSelection.subject" subject),
      .setStatus .untapped (.laterMention "repeatedSelection.subject" subject)] := rfl
end DispatchContract

open Semantics in
theorem generatedActorKeepsDynamicHandoffReads :
    SylFixture.Semantics.Definitions.actorAcrossControllerHandoff =
      .ownScope (some (.action "actorAcrossControllerHandoff"))
        (Templates.instruct
          (Templates.controllerOf (.firstMention "actorAcrossControllerHandoff.actor" Templates.theActor))
          (Templates.instruct .actor
            (Definitions.sacrifice (.laterMention "actorAcrossControllerHandoff.actor" Templates.theActor)))) := rfl

private def namesFor (root : Lean.Name) (source : String) : Option (List Lean.Name) := do
  let declarations ← (SylGen.parse "naming.syl" source).toOption
  return declarations.toList.map (fun d => SylGen.declarationName root d declarations)

example : namesFor .anonymous
    "template item: Int { body = 1 } template item: Bool { body = true }" =
    .some [`Semantics.Templates.Int.item, `Semantics.Templates.Bool.item] := by native_decide

/-- The same overloaded declarations receive the same names in either discovery order. -/
theorem sharedNamesDoNotDependOnLoadOrder :
    (namesFor .anonymous
      "template item: Int { body = 1 } template item: Bool { body = true }").map List.reverse =
    namesFor .anonymous
      "template item: Bool { body = true } template item: Int { body = 1 }" := by native_decide

/-- A body-less primitive has the same placement under either root. -/
theorem samePrimitivePlacementForEveryRoot :
    namesFor .anonymous "primitive lit(value: Int): Amount" =
      .some [`Semantics.Templates.lit] ∧
    namesFor `Fixture "primitive lit(value: Int): Amount" =
      .some [`Fixture.Semantics.Templates.lit] := by native_decide

theorem parameterAbilityWordKeepsItsOuterHeading (ability : Semantics.Ability) :
    SylFixture.Semantics.Definitions.threshold (.italicHead (.abilityWord (.named "inner")) ability) =
      .italicHead (.abilityWord (.named "threshold"))
        (.italicHead (.abilityWord (.named "inner")) ability) := rfl
