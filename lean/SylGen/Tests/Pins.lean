import Semantics.Cards.Data.Shard00
import Semantics.Cards.Data.Shard03
import Semantics.Cards.Data.Shard04
import Semantics.Cards.Data.Shard05
import Semantics.Cards.Data.Shard11
import Semantics.Cards.Data.Shard12
import SylFixtures
import NormalFixtures
import SylGen.Parser
import Semantics.Cards.Data.Shard06

example : @SylFixture.Semantics.Primitives.Action.doForEachKind =
    @Semantics.Primitives.Action.doForEachKind := rfl
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
      Templates.and [
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
      Primitives.Action.doForEachKind .color (some (.firstMention "eachKind.who" who)) .color
        (Templates.instruct (.laterMention "eachKind.who" who) (Templates.draw 1)) := rfl

example : SylFixture.Semantics.Templates.omittedArgument = (-1 : Int) := rfl
example : SylFixture.Semantics.Templates.presentAtom =
    some SylFixture.Semantics.Templates.thisCreature := rfl
example : SylFixture.Semantics.Templates.optionalList =
    some [SylFixture.Semantics.Definitions.flying] := rfl

example : SylFixture.Semantics.Cards.Forest = Semantics.Cards.Forest.card := rfl
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

example : SylFixture.Semantics.Cards.«Branchloft Pathway // Boulderloft Pathway» = Semantics.Cards.«Branchloft Pathway // Boulderloft Pathway».card := rfl

example : SylFixture.Semantics.Cards.«Merfolk Secretkeeper // Venture Deeper» = Semantics.Cards.«Merfolk Secretkeeper // Venture Deeper».card := rfl

example : SylFixture.Semantics.Cards.«Blitz Automaton» = Semantics.Cards.«Blitz Automaton».card := rfl

example : SylFixture.Semantics.Cards.«Student of Warfare» = Semantics.Cards.«Student of Warfare».card := rfl

example : SylFixture.Semantics.Cards.«Profit // Loss» = Semantics.Cards.«Profit // Loss».card := rfl

example : SylFixture.Semantics.Cards.«Glassworks // Shattered Yard» = Semantics.Cards.«Glassworks // Shattered Yard».card := rfl

example : SylFixture.Semantics.Cards.«Neglected Heirloom // Ashmouth Blade» = Semantics.Cards.«Neglected Heirloom // Ashmouth Blade».card := rfl

example : SylFixture.Semantics.Cards.«Bushi Tenderfoot // Kenzo the Hardhearted» = Semantics.Cards.«Bushi Tenderfoot // Kenzo the Hardhearted».card := rfl

example : SylFixture.Semantics.Definitions.fight.deed = Semantics.Definitions.fight.deed := rfl
example : SylFixture.Semantics.Definitions.flying.label = Semantics.Definitions.flying.label := rfl
example : SylFixture.Semantics.Definitions.bear_definition = Semantics.Definitions.bear_definition := rfl
example : SylFixture.Semantics.Definitions.haunting_definition = Semantics.Definitions.haunting_definition := rfl

example : NormalFixture.Semantics.Templates.shared = (-1 : Int) := rfl
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

namespace Round2Expected
open Semantics Semantics.Templates
template recordOrder (who : NounPhrase) : DidClause := {
  body := { action := instruct who (draw 1), within := none, subj := actor who }
}
ability_word alreadyWord (ability : Ability) : Ability := {
  label := "outer", body := italicHead (.abilityWord (.named "inner")) ability
}
ability_word repeatedAbilityWord (who : NounPhrase) : Ability := {
  label := "repeated", body := spell (simultaneously [instruct who (draw 1), instruct who (draw 1)])
}
end Round2Expected
example : SylFixture.Semantics.Templates.recordOrder = Round2Expected.recordOrder := rfl
example : SylFixture.Semantics.Definitions.alreadyWord = Round2Expected.alreadyWord := rfl
example : SylFixture.Semantics.Definitions.repeatedAbilityWord = Round2Expected.repeatedAbilityWord := rfl

open Lean in
run_cmd do
  let env ← getEnv
  let entries := Semantics.Authoring.expectedTypeDispatchExt.getState env
  unless ((entries.find? `fight).getD []).any
      (·.declaration == `SylFixture.Semantics.Definitions.fight) do
    throwError "generated declaration missing from dispatch registry"
  unless (Semantics.Authoring.canonCardsExt.getState env).any
      (·.declaration == `SylFixture.Semantics.Cards.Forest) do
    throwError "generated card missing from canon registry"
  for name in [`SylFixture.Semantics.Definitions.flying.term,
      `NormalFixture.Semantics.Definitions.withoutDeed.deed,
      `NormalFixture.Semantics.Definitions.abilityDeed.deed] do
    if env.contains name then throwError "unexpected generated constant {name}"
  unless Semantics.Authoring.templateAttribute.hasTag env
      `SylFixture.Semantics.Templates.yourHand do throwError "missing template attribute"
  unless Semantics.Authoring.definitionAttribute.hasTag env
      `SylFixture.Semantics.Definitions.fight do throwError "missing definition attribute"
  unless Semantics.Authoring.bundleRegistryAttribute.hasTag env
      `SylFixture.Semantics.Definitions.aegisCounter do throwError "missing registry attribute"
  unless ((entries.find? `in).getD []).isEmpty do
    throwError "unexpected surface-spelling dispatch entry"
  unless ((entries.find? `in_).getD []).any
      (·.declaration == `NormalFixture.Semantics.Templates.in_) do
    throwError "missing Lean-spelling dispatch entry"
  unless Semantics.Authoring.primitiveAttribute.hasTag env
      `SylFixture.Semantics.Primitives.Action.doForEachKind do
    throwError "missing primitive attribute"

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
