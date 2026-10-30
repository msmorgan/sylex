import SylFixtures
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
