import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Copy

Port of `idris/src/Experimental/Cards/Copy.idr`: the printed cards of the Copy family. A bench
item that is an ability, an action, or a static clause rather than a whole card is a plain
definition paired with an `ok…` theorem that runs the checker on it, as its Idris elaboration
did.
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (a)
open Semantics.Templates.Predicate (and)
open Semantics.Templates.Option.Amount (stat)
open Semantics.Templates.Location (and)
open Semantics.Templates.NounPhrase (and)
open Semantics.Templates.TokenSpec (written)

open Semantics.Definitions.CardType (planeswalker)
open Semantics.Templates.AbilityLost (written)
open Semantics.Templates.Action (copy target)
open Semantics.Templates.CharacteristicEdit (stat)
open Semantics.Templates.Condition (and)
open Semantics.Templates.DetPhrase (a)
open Semantics.Templates.NounWord (planeswalker)

namespace Semantics.Cards

/-- Display of Power: "this spell can't be copied". -/
def displayOfPowerCopyLock : StaticSpec := cant any (copyOnStack this_ anyAmount)
theorem okDisplayOfPowerCopyLock : StaticSpec.check (DRS.ofList []) displayOfPowerCopyLock = [] := by decide

/-- Frontline Heroism -/
def frontlineHeroismCopy : Ability :=
  whenever
    (does (Semantics.Subject.actor you) (cast (a (and [ isSpell,
                 targeting (a (and [isCreature,
                   youControl]))
                   .soleTarget ]))))
    (sequentially
      [ create 1 (written
          { characteristics :=
            { colors := [.red], types := [.creature], subtypes := [soldier],
              text := [haste], power := stat 1, toughness := stat 1 } }),
        copyOnStack (that .spell),
        copyTargets (that .copy) thatToken ])
theorem okFrontlineHeroismCopy : Ability.check (DRS.ofList []) frontlineHeroismCopy = [] := by decide

/-- Flawless Forgery -/
def flawlessForgeryLine : Action :=
  sequentially
    [ exile (target (and [isInstantOrSorcery, in_ (graveyardOf anOpponent)])),
      copy .fromCardZone (that card) 1 [],
      may (cast (that .copy) (payment := withoutPaying)) ]
theorem okFlawlessForgeryLine : Action.check (DRS.ofList []) flawlessForgeryLine = [] := by decide

/-- Echo Mage, level 4+ -/
def echoMagesFourthLevel : Ability :=
  activated (compound [(Semantics.Cost.mana [pip .blue, pip .blue]), tapSymbol])
    (sequentially
      [ copyOnStack (target (and [isInstantOrSorcery, isSpell])) 2,
        may (chooseNewTargets (those .copy)) ])
theorem okEchoMagesFourthLevel : Ability.check (DRS.ofList []) echoMagesFourthLevel = [] := by decide

/-- Mister Fantastic -/
def misterFantasticCopy : Ability :=
  activated (compound [(Semantics.Cost.mana [pip .red, pip .green, pip .white, pip .blue]), tapSymbol])
    (sequentially
      [ copyOnStack (target (and [abilityHead .anyTriggered, youControl])) 2,
        may (chooseNewTargets (those (.copied .ability))) ])
theorem okMisterFantasticCopy : Ability.check (DRS.ofList []) misterFantasticCopy = [] := by decide

/-- Rowan's Talent -/
def rowansTalentCopy : Ability :=
  whenever
    (does (Semantics.Subject.actor you) (activate (a (and [ abilityHead .loyalty,
                 abilityOf (enchanted planeswalker) ]))))
    (sequentially
      [ copyOnStack (that .ability),
        may (chooseNewTargets (that (.copied .ability))) ])
theorem okRowansTalentCopy : Ability.check (DRS.ofList []) rowansTalentCopy = [] := by decide

/-- Pyromancer's Goggles -/
def pyromancersGogglesMana : Ability :=
  activated tapSymbol
    (addMana 1 (manaOf [[.of .red]])
      [ whenSpentOn .triggersThen false
          (a (and [hasColor .red, isInstantOrSorcery, isSpell]))
          (sequentially
            [ copyOnStack (that .spell),
              may (chooseNewTargets (that .copy)) ]) ])
theorem okPyromancersGogglesMana : Ability.check (DRS.ofList []) pyromancersGogglesMana = [] := by decide

end Semantics.Cards
