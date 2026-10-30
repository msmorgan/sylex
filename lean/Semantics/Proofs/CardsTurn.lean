import Semantics.Cards.Data.Fixtures
import Semantics.Check
import Semantics
import Semantics.Definitions
import Semantics.Check.Card

-- Canon cards are `noncomputable` data (`Semantics.Cards.Data`); so is what reads them.
noncomputable section

/-!
# Semantics.Cards.Turn

Port of `idris/src/Experimental/Cards/Turn.idr`: the printed cards of the Turn family (turn
parts, extra turns, additional phases, cast windows) and the bench items beside them.

Not ported: `fracturedPowerstonePlanarRoll` (the planar die is Planechase, out of scope).
`shapeshifter`'s printed `*`/`7-*` box is `none` under the printed-star ruling; its
Characteristic-Defining Ability sets it.
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (a the)
open Semantics.Templates.Predicate (and or not did was)
open Semantics.Templates.ManaSymbol (generic)
open Semantics.Templates.Option.Amount (stat)
open Semantics.Templates.Location (and or not)
open Semantics.Templates.NounPhrase (and or)
open Semantics.Templates.TokenSpec (written)
open Semantics.Templates.Timing (duringPart not)

open Semantics.Definitions.CardType (creature)
open Semantics.Templates.AbilityLost (written)
open Semantics.Templates.Action (target)
open Semantics.Templates.CharacteristicEdit (stat)
open Semantics.Templates.Condition (and did duringPart not or was)
open Semantics.Templates.CostShift (generic)
open Semantics.Templates.DetPhrase (a the)
open Semantics.Templates.NounWord (creature)

namespace Semantics.Cards

/-- Through the Breach Splice -/
def throughTheBreach : Action :=
  may
    (putOntoBattlefield (a (and [isCreature, in_ yourHand])))
    (sequentially
      [ gainHaste (that creature),
        atNext .endStep (sacrifice (that creature)) ])
theorem okThroughTheBreach : Action.check (DRS.ofList []) throughTheBreach = [] := by decide

def turnToMist : Action :=
  sequentially
    [ exile (target isCreature),
      atNext .endStep (returnToBattlefield (that card) [(under (ownerOf it))]) ]
theorem okTurnToMist : Action.check (DRS.ofList []) turnToMist = [] := by decide

def voyagerStaff : Ability :=
  activated (compound [(Semantics.Cost.mana [generic 2]), (Semantics.Cost.perform (sacrifice thisArtifact))])
    (sequentially
      [ exile (target isCreature),
        atNext .endStep
          (returnToBattlefield (the (and [was (exile which), isCard])) [(under (ownerOf it))]) ])
theorem okVoyagerStaff : Ability.check (DRS.ofList []) voyagerStaff = [] := by decide

def staffOfNin : Ability :=
  at_ (beginningOfYour .upkeep) (draw 1)
theorem okStaffOfNin : Ability.check (DRS.ofList []) staffOfNin = [] := by decide

def silentAssassin : Ability :=
  activated (Semantics.Cost.mana [generic 3, pip .black])
    (atNext .endOfCombat (destroy (target (and [isBlocking, isCreature]))))
theorem okSilentAssassin : Ability.check (DRS.ofList []) silentAssassin = [] := by decide

def kjeldoranFrostbeast : Ability :=
  at_ (beginningOfEach .endOfCombat)
    (destroy (allOf (and [ isCreature,
                            or [ inCombat .blockerOf thisCreature,
                                  inCombat .blockedBy thisCreature ] ])))
theorem okKjeldoranFrostbeast : Ability.check (DRS.ofList []) kjeldoranFrostbeast = [] := by decide

/-- Tippy-Toe, Terrific Partner -/
def tippyToe : Ability :=
  at_ (beginningOfYour .endStep) (if_ := some (did (Semantics.Subject.actor you) (gainLife anyAmount) (.duration soFarThisTurn none)))
    (draw 1)
theorem okTippyToe : Ability.check (DRS.ofList []) tippyToe = [] := by decide

def brazenCannonade : Ability :=
  raid
    (at_ (beginningOfYour .postcombatMain)
      (if_ := some (did (Semantics.Subject.actor you) attack (Semantics.LookbackSpan.duration (soFarThisTurn) none)))
      (exile (topSlice 1)))
theorem okBrazenCannonade : Ability.check (DRS.ofList []) brazenCannonade = [] := by decide

def fourKnocks : Ability :=
  at_ (beginningOfYour .firstMain) (draw 1)
theorem okFourKnocks : Ability.check (DRS.ofList []) fourKnocks = [] := by decide

def hammerOfBogardan : Ability :=
  activatedOnlyDuring (Semantics.Cost.mana [generic 2, pip .red, pip .red, pip .red]) (returnToHand this_ yourGraveyard)
    (duringPart .upkeep you)
theorem okHammerOfBogardan : Ability.check (DRS.ofList []) hammerOfBogardan = [] := by decide

/-- Damia, Sage of Stone -/
def damia : Ability :=
  at_ (beginningOfYour .upkeep)
    (if_ := compareAmt (countOf (cardIn yourHand)) .less 7)
    (draw theDifference)
theorem okDamia : Ability.check (DRS.ofList []) damia = [] := by decide

/-- Ajani, Adversary of Tyrants -/
def ajanisEmblem : Action :=
  getEmblem
    [ at_ (beginningOfYour .endStep)
        (create 3 (written
          { characteristics :=
            { colors := [.white], types := [.creature], subtypes := [cat],
              text := [lifelink], power := stat 1, toughness := stat 1 } })) ]
theorem okAjanisEmblem : Action.check (DRS.ofList []) ajanisEmblem = [] := by decide

/-- Saheeli Rai -/
def saheelisCopy : Action :=
  sequentially
    [ create 1
        (copyOf (target (and [or [isArtifact, isCreature], youControl]))
          [isAlso (some [.artifact])])
        [],
      gainHaste thatToken,
      atNext .endStep (exile it) ]
theorem okSaheelisCopy : Action.check (DRS.ofList []) saheelisCopy = [] := by decide

def finalFortuneExtraTurn : Action := (match (Semantics.Cards.«Final Fortune») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ (Action.sequentially (List.cons part _))) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okFinalFortuneExtraTurn : Action.check (DRS.ofList []) finalFortuneExtraTurn = [] := by decide
def finalFortuneThatTurn : TurnRef := (match (Semantics.Cards.«Final Fortune») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ (Action.sequentially (List.cons _ (List.cons (Action.delay
        (GameEvent.beginningOf _ _ part) _ _ _) _)))) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okFinalFortuneThatTurn :
    TurnRef.check ((stepAction 512 .postEvent .obj (DRS.ofList []) finalFortuneExtraTurn).1) finalFortuneThatTurn = [] := by
  decide

def fullThrottleFirstLine : Action := additionalPhase .combat (some .mainPhase) 2
theorem okFullThrottleFirstLine : Action.check (DRS.ofList []) fullThrottleFirstLine = [] := by decide
def yshtolaAdditionalEndStep : Action := additionalStep .endStep none 1
theorem okYshtolaAdditionalEndStep : Action.check (DRS.ofList []) yshtolaAdditionalEndStep = [] := by decide

def ninthDoctorAdditionalUpkeep : Action := additionalStep .upkeep none 1
theorem okNinthDoctorAdditionalUpkeep :
    Action.check (DRS.ofList []) ninthDoctorAdditionalUpkeep = [] := by decide

def chainVeilEndStep : Ability :=
  at_ (beginningOfYour .endStep)
    (if_ := some (not (did (Semantics.Subject.actor (you)) (activate (a
      (and [abilityHead .loyalty,
      abilityOf (a isPlaneswalker)]))) (.duration soFarThisTurn none))))
    (loseLife 2)
theorem okChainVeilEndStep : Ability.check (DRS.ofList []) chainVeilEndStep = [] := by decide

def selfSacrificeThenExile : Ability :=
  activated (Semantics.Cost.perform (sacrifice thisArtifact))
    (sequentially
      [ exile (target isCreature),
        atNext .endStep (putOntoBattlefield (that card)) ])
theorem okSelfSacrificeThenExile : Ability.check (DRS.ofList []) selfSacrificeThenExile = [] := by decide

def raphaelAdditionalCombat : Action := additionalPhase .combat (some .combat) 1
theorem okRaphaelAdditionalCombat : Action.check (DRS.ofList []) raphaelAdditionalCombat = [] := by decide
/-- Karn Liberated -/
def karnRestart : Action := restartGame
theorem okKarnRestart : Action.check (DRS.ofList []) karnRestart = [] := by decide

/-! ## Enchant creature put onto the battlefield with this

Each one's entering ability gives it an enchant ability naming the creature that ability puts
onto the battlefield, puts the creature there, attaches the Aura to it, and creates a delayed
trigger that sacrifices the creature when the Aura leaves [CR#603.7a,603.7c]. The granted enchant ability is linked to the ability that grants it, so "put
onto the battlefield with [this object]" reads the creature that ability puts, though a later step
puts it [CR#607.1a,607.2c]. -/

/-- "creature card in a graveyard" -/
def creatureCardInAGraveyard : Predicate := (match (Semantics.Cards.«Necromancy») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons _
        (List.cons (Action.move (NounPhrase.described _ part) _ _ _ _) _)))) _)) => some part
      | _ => none
    | _ => none).get (by decide)

/-- "it loses 'enchant creature card in a graveyard' and gains 'enchant creature put onto the
battlefield with this Aura.'" -/
def swapsEnchantForLinkedEntry : Action :=
  (match (Semantics.Cards.«Dance of the Dead») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons
        part _))) _)) => some part
      | _ => none
    | _ => none).get (by decide)

end Semantics.Cards

end
