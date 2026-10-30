import Semantics.Cards.Data.Fixtures
import Semantics.Check
import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsHelpers

-- Canon cards are `noncomputable` data (`Semantics.Cards.Data`); so is what reads them.
noncomputable section


/-!
# Semantics.Cards.Cost

Port of `idris/src/Experimental/Cards/Cost.idr`: the printed cards of the Cost family
(activation costs, cost changes, alternative costs, the cost's X) and the bench items beside
them; the file's `Refl` pins on `costLetters` are theorems.

Not ported: `missyChaosBranch` ("chaos ensues" is Planechase, out of scope).
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (a each)
open Semantics.Templates.Predicate (and or not did was)
open Semantics.Templates.ManaSymbol (generic)
open Semantics.Templates.Location (and or not)
open Semantics.Templates.NounPhrase (and or)
open Semantics.Templates.TokenSpec (written)
open Semantics.Templates.Timing (not)

open Semantics.Definitions.CardType (creature)
open Semantics.Templates.AbilityLost (written)
open Semantics.Templates.Action (target)
open Semantics.Templates.Condition (and did not or was)
open Semantics.Templates.CostShift (generic)
open Semantics.Templates.DetPhrase (a each)
open Semantics.Templates.NounWord (creature)

namespace Semantics.Cards

def masterDecoy : Ability :=
  activated (compound [(Semantics.Cost.mana ([pip .white])), tapSymbol]) (tap (target isCreature))
theorem okMasterDecoy : Ability.check (DRS.ofList []) masterDecoy = [] := by decide
def cycling : Ability :=
  activated (compound [(Semantics.Cost.mana ([generic 2])), (Semantics.Cost.perform (discard this_))]) (draw 1)
theorem okCycling : Ability.check (DRS.ofList []) cycling = [] := by decide
def merrowGrimeblotter : Ability :=
  activated (compound [(Semantics.Cost.mana ([generic 1, hybridPip .blue .black])), untapSymbol])
    (get (target isCreature) (down 2) (down 0) untilEndOfTurn)
theorem okMerrowGrimeblotter : Ability.check (DRS.ofList []) merrowGrimeblotter = [] := by decide
def phyrexianSnowcrusher : Ability :=
  activated (Semantics.Cost.mana ([generic 1, .snow]))
    (get thisCreature (up 1) (up 0) untilEndOfTurn)
theorem okPhyrexianSnowcrusher : Ability.check (DRS.ofList []) phyrexianSnowcrusher = [] := by decide
def havocSower : Ability :=
  activated (Semantics.Cost.mana ([generic 1, colorlessPip]))
    (get thisCreature (up 2) (up 1) untilEndOfTurn)
theorem okHavocSower : Ability.check (DRS.ofList []) havocSower = [] := by decide
/-- Erebos, God of the Dead -/
def erebos : Ability :=
  activated (compound [(Semantics.Cost.mana ([generic 1, pip .black])), (Semantics.Cost.perform (payLife 2))]) (draw 1)
theorem okErebos : Ability.check (DRS.ofList []) erebos = [] := by decide
def baskingRootwalla : Ability :=
  activatedOnlyOnce (Semantics.Cost.mana ([generic 1, pip .green]))
    (get thisCreature (up 2) (up 2) untilEndOfTurn) oncePerTurn
theorem okBaskingRootwalla : Ability.check (DRS.ofList []) baskingRootwalla = [] := by decide
def securityDetail : Ability :=
  activatedOnlyOnceIf (Semantics.Cost.mana ([pip .white, pip .white]))
    (create 1 (Semantics.TokenSpec.written ((creatureToken 1 1 [.white] [soldier])))) oncePerTurn
    (not (exists_ (and [isCreature, youControl])))
theorem okSecurityDetail : Ability.check (DRS.ofList []) securityDetail = [] := by decide
def aphettoAlchemist : Ability :=
  activated tapSymbol (untap (target (or [isArtifact, isCreature])))
theorem okAphettoAlchemist : Ability.check (DRS.ofList []) aphettoAlchemist = [] := by decide

/-- Sisters of Stone Death -/
def sistersOfStoneDeathRecall : Ability :=
  activated (Semantics.Cost.mana ([generic 2, pip .black]))
    (putOntoBattlefieldUnderYourControl (a (and [isCreature, in_ (exiledWith thisCreature)])))
theorem okSistersOfStoneDeathRecall : Ability.check (DRS.ofList []) sistersOfStoneDeathRecall = [] := by decide
/-- Synod Sanctum -/
def synodSanctumReturn : Ability :=
  activated (compound [(Semantics.Cost.mana ([generic 2])), (Semantics.Cost.perform (sacrifice thisArtifact))])
    (returnToBattlefield (allOf (cardIn (exiledWith thisArtifact))) [(under you)])
theorem okSynodSanctumReturn : Ability.check (DRS.ofList []) synodSanctumReturn = [] := by decide

def barlsCage : Ability :=
  activated (Semantics.Cost.mana ([generic 3])) (skipUntap (target isCreature) 1)
theorem okBarlsCage : Ability.check (DRS.ofList []) barlsCage = [] := by decide
def vodalianIllusionist : Ability :=
  activated (compound [(Semantics.Cost.mana ([pip .blue, pip .blue])), tapSymbol])
    (instruct (target isCreature) phaseOut)
theorem okVodalianIllusionist : Ability.check (DRS.ofList []) vodalianIllusionist = [] := by decide
def witchsMist : Ability :=
  activated (compound [(Semantics.Cost.mana ([generic 2, pip .black])), tapSymbol])
    (destroy (target (and [isCreature, was (dealDamage anyAmount which) (Semantics.LookbackSpan.duration (soFarThisTurn) none)])))
theorem okWitchsMist : Ability.check (DRS.ofList []) witchsMist = [] := by decide
def goadTargetCreature : Ability :=
  activated (compound [(Semantics.Cost.mana ([generic 3])), tapSymbol])
    (gainDesignation (target isCreature) goaded)
theorem okGoadTargetCreature : Ability.check (DRS.ofList []) goadTargetCreature = [] := by decide
/-- Krenko, Mob Boss -/
def krenko : Ability :=
  activated tapSymbol
    (sequentially
      [ create (letter .x) (Semantics.TokenSpec.written ((creatureToken 1 1 [.red] [goblin]))),
        define .x (countOf (and [(Semantics.Predicate.hasSubtype (goblin)), youControl])) ])
theorem okKrenko : Ability.check (DRS.ofList []) krenko = [] := by decide
/-- Dokai, Weaver of Life -/
def dokai : Ability :=
  activated (compound [(Semantics.Cost.mana ([generic 4, pip .green, pip .green])), tapSymbol])
    (sequentially
      [ create 1 (Semantics.TokenSpec.written ((creatureTokenOf (letter .x) (letter .x) [.green] [elemental]))),
        define .x (countOf (and [isLand, youControl])) ])
theorem okDokai : Ability.check (DRS.ofList []) dokai = [] := by decide

def ancientStoneIdol : Ability :=
  static (costShift this_ (less (forEach 1 (and [isCreature, isAttacking]))))
theorem okAncientStoneIdol : Ability.check (DRS.ofList []) ancientStoneIdol = [] := by decide

def nullhideFerox : Ability :=
  activated (Semantics.Cost.mana ([generic 2]))
    (establishThroughout (allAbilityLoss thisCreature) untilEndOfTurn)
theorem okNullhideFerox : Ability.check (DRS.ofList []) nullhideFerox = [] := by decide

/-- Saheeli, Filigree Master -/
def saheelisEmblem : Action :=
  getEmblem
    [ static (getsPt (allOf (and [isArtifact, isCreature, youControl]))
        (up 1) (up 1)),
      static ((costShift (allOf (and [isArtifact, isSpell, did (Semantics.Subject.actor (you)) (cast which) thisObject])) (less 1))) ]
theorem okSaheelisEmblem : Action.check (DRS.ofList []) saheelisEmblem = [] := by decide

/-- Elspeth's Talent -/
def elspethsTalentGrant : StaticSpec :=
  (match (Semantics.Cards.«Elspeth's Talent») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.static part) _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okElspethsTalentGrant : StaticSpec.check (DRS.ofList []) elspethsTalentGrant = [] := by decide

/-- Amoeboid Changeling -/
def amoeboidChangelingTypeAbilities : List Ability :=
  [ activated tapSymbol
      (establishThroughout (qualityChange (target isCreature) .adds (everyTypeOf .creature))
         untilEndOfTurn),
    activated tapSymbol
      (establishThroughout (qualityChange (target isCreature) .loses (everyTypeOf .creature))
         untilEndOfTurn) ]
theorem okAmoeboidChangelingTypeAbilities :
    Ability.checkText (DRS.ofList []) amoeboidChangelingTypeAbilities = [] := by decide

/-- Boom Scholar -/
def boomScholarExhaustDiscount : Ability :=
  static (costShift
    (allOf (and [ abilityHead (.keyword exhaust.label),
                   abilityOf (allOf (and [isPermanent, isOther, youControl])) ]))
    (less 2))
theorem okBoomScholarExhaustDiscount : Ability.check (DRS.ofList []) boomScholarExhaustDiscount = [] := by decide
/-- Hulk, Gamma Goliath -/
def hulkPowerUpDiscount : Ability :=
  static (costShift
    (allOf (and [ abilityHead (.keyword powerUp.label),
                   abilityOf (allOf (and [isCreature, isOther, youControl])) ]))
    (less 3))
theorem okHulkPowerUpDiscount : Ability.check (DRS.ofList []) hulkPowerUpDiscount = [] := by decide

/-- Fervent Champion -/
def ferventChampionEquipDiscount : Ability :=
  static (costShift
    (allOf (and [abilityHead (.keyword equip.label), did (Semantics.Subject.actor (you)) (activate which) thisObject,
        targeting thisCreature .someTarget]))
    (less 3))
theorem okFerventChampionEquipDiscount : Ability.check (DRS.ofList []) ferventChampionEquipDiscount = [] := by decide

def oppressiveRaysLine : StaticSpec :=
  (match (Semantics.Cards.«Oppressive Rays») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons (Ability.static part) _))) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okOppressiveRaysLine : StaticSpec.check (DRS.ofList []) oppressiveRaysLine = [] := by decide

def hurrJackalAbility : Ability :=
  activated tapSymbol
    (establishThroughout (cant any (regeneration (target isCreature))) thisTurn)
theorem okHurrJackalAbility : Ability.check (DRS.ofList []) hurrJackalAbility = [] := by decide

/-- Void Maw -/
def voidMawPutCost : Ability :=
  activated (Semantics.Cost.perform ((putInto (a (in_ (exiledWith thisCreature))) graveyard)))
    (get thisCreature (up 2) (up 2) untilEndOfTurn)
theorem okVoidMawPutCost : Ability.check (DRS.ofList []) voidMawPutCost = [] := by decide
/-- Ghor-Clan Rampager -/
def ghorClanRampager : Ability :=
  bloodrush
    (activated (compound [(Semantics.Cost.mana ([pip .red, pip .green])), (Semantics.Cost.perform (discard this_))])
      (establishFor (target (and [isCreature, isAttacking]))
        [getsPt it (up 4) (up 4), abilityGrant it trample]
        untilEndOfTurn))
theorem okGhorClanRampager : Ability.check (DRS.ofList []) ghorClanRampager = [] := by decide
/-- Tymora's Invoker -/
def tymorasInvoker : Ability :=
  flavorWord "Sleight of Hand" (activated (Semantics.Cost.mana ([generic 8])) (draw 2))
theorem okTymorasInvoker : Ability.check (DRS.ofList []) tymorasInvoker = [] := by decide
/-- Skyblade's Boon -/
def skybladesBoonReturn : Ability :=
  activatedOnlyIf (Semantics.Cost.mana ([generic 2, pip .white])) (returnToHand this_)
    (or [is this_ (in_ battlefield), is this_ (cardIn yourGraveyard)])
theorem okSkybladesBoonReturn : Ability.check (DRS.ofList []) skybladesBoonReturn = [] := by decide
/-- Bamboozling Beeble -/
def bamboozlingBeebleIgnore : Ability :=
  activated (compound [(Semantics.Cost.mana ([generic 1])), tapSymbol])
    (replaceNextEvent (would ((Semantics.Subject.actor (target isPlayer))) (rollDice anyAmount .anyDie))
      (instead (sequentially
        [ instruct they (rollDice (plus thatMuch 1) .thoseDice),
          ignoreOutcomes (chosenResults you 1) ]))
      thisTurn)
theorem okBamboozlingBeebleIgnore : Ability.check (DRS.ofList []) bamboozlingBeebleIgnore = [] := by decide
/-- General Tazri's pump -/
def generalTazriPump : Ability :=
  activated (Semantics.Cost.mana ([pip .white, pip .blue, pip .black, pip .red, pip .green]))
    (sequentially
      [ get (each (and [isCreature, (Semantics.Predicate.hasSubtype (ally)), youControl]))
          (up (letter .x)) (up (letter .x)) untilEndOfTurn,
        define .x (distinctCount .color (those creature)) ])
theorem okGeneralTazriPump : Ability.check (DRS.ofList []) generalTazriPump = [] := by decide
/-- Diplomatic Escort -/
def diplomaticEscortLine : Ability :=
  activated (compound [(Semantics.Cost.mana ([pip .blue])), tapSymbol, (Semantics.Cost.perform (discard (a (cardIn yourHand))))])
    (counter
      (target (and [or [isSpell, abilityHead .anyOnStack], targeting (a isCreature) .someTarget])))
theorem okDiplomaticEscortLine : Ability.check (DRS.ofList []) diplomaticEscortLine = [] := by decide

/-- Prosperity -/
theorem prosperityCostLetters : costLetters [.variable, pip .blue] = [letterB .x] := rfl
def prosperityTextLetter : Amount := (match (Semantics.Cards.«Prosperity») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ (Action.instruct _ (Action.draw (Quantity.exactlyOf part)))) _)
        => some part
      | _ => none
    | _ => none).get (by decide)
theorem prosperityTextReadsCostLetter :
    let context := DRS.ofList (costLetters [.variable, pip .blue])
    let out := (stepAmount 64 context prosperityTextLetter).1
    out.referents.filter (fun ref => ref.id ≥ context.nextId) = [] := by decide
theorem noVariableSymbolNoLetter : costLetters [generic 1, pip .blue] = [] := by decide
theorem noCostNoLetter : costLetters none = [] := by decide

/-- Doc Aurlock, Grizzled Genius -/
def docAurlockCost : StaticSpec :=
  (costShift (allOf (and [isSpell, did (Semantics.Subject.actor (you)) (cast which (from_ := some (or [yourGraveyard, exileZone]))) thisObject])) (less 2))
theorem okDocAurlockCost : StaticSpec.check (DRS.ofList []) docAurlockCost = [] := by decide
/-- Obelisk of Undoing -/
def obeliskOfUndoing : Ability :=
  activated (compound [(Semantics.Cost.mana ([generic 6])), tapSymbol])
    (returnToHand (target (and [isPermanent, youOwn, youControl])))
theorem okObeliskOfUndoing : Ability.check (DRS.ofList []) obeliskOfUndoing = [] := by decide

/-- Cavern-Hoard Dragon -/
def cavernHoardDragonRider : Ability :=
  static (conjunction [ costShift this_ (less (letter .x)),
      letterDefinition .x
        (aggregateOver .max isOpponent (countOf (and [isArtifact, controlledBy they]))) ])
theorem okCavernHoardDragonRider : Ability.check (DRS.ofList []) cavernHoardDragonRider = [] := by decide
/-- Shadowspear -/
def shadowspearStrip : Ability :=
  activated (Semantics.Cost.mana ([generic 1]))
    (establishThroughout (abilityLoss (allOf (and [isPermanent, opponentsControl]))
        [written hexproof, written
           indestructible]) untilEndOfTurn)
theorem okShadowspearStrip : Ability.check (DRS.ofList []) shadowspearStrip = [] := by decide
/-- Shay Cormac -/
def shayCormacStrip : Ability :=
  activated (Semantics.Cost.mana ([generic 1]))
    (establishThroughout (abilityLoss (allOf (and [isPermanent, opponentsControl]))
        [ written hexproof, written indestructible, abilities protectionAbilities,
          written shroud, abilities wardAbilities ]) untilEndOfTurn)
theorem okShayCormacStrip : Ability.check (DRS.ofList []) shayCormacStrip = [] := by decide

/-- Nesting Dragon -/
def nestingDragonInnerToken : Ability :=
  activated (Semantics.Cost.mana ([pip .red]))
    (establishThroughout
       (conjunction [ getsPt thisToken (up 1) (up 0) ]) untilEndOfTurn)
theorem okNestingDragonInnerToken : Ability.check (DRS.ofList []) nestingDragonInnerToken = [] := by decide

/-- Alluring Suitor // Deadly Dancer -/
def deadlyDancerPump : Ability :=
  activated (Semantics.Cost.mana ([pip .red, pip .red]))
    (establishThroughout
       (getsPt (eachOf (both thisCreature (target (and [isCreature, isOther]))))
          (up 1) (up 0)) untilEndOfTurn)
theorem okDeadlyDancerPump : Ability.check (DRS.ofList []) deadlyDancerPump = [] := by decide

def bareTextLetter : Amount := letter .x
theorem textAloneOnceMintedItsOwnLetter :
    (stepAmount 64 (DRS.ofList []) bareTextLetter).1.referents = [letterB .x] := by decide

end Semantics.Cards

end
