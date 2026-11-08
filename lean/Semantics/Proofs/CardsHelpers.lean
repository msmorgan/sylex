import Semantics.Cards.Data.Fixtures
import Semantics.Check
import Semantics
import Semantics.Definitions
import Semantics.Check.Card

-- Canon cards are `noncomputable` data (`Semantics.Cards.Data`); so is what reads them.
noncomputable section

/-!
# Semantics.Proofs.CardsHelpers

Shared bench helpers extracted from `Cards*.lean` modules. These break the cross-module
`Cards*` dependency chain and permit downstream proof suites to import only the shared helpers
rather than whole card test suites.
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (a counted)
open Semantics.Templates.Predicate (and or)
open Semantics.Templates.ManaSymbol (generic)
open Semantics.Templates.Location (and or)
open Semantics.Templates.NounPhrase (and or)
open Semantics.Templates.StaticSpec (replace)

open Semantics.Definitions.Ability (crew saddle)
open Semantics.Definitions.CardType (creature)
open Semantics.Templates.Action (crew replace saddle target)
open Semantics.Templates.Condition (and or)
open Semantics.Templates.CostShift (generic)
open Semantics.Templates.DetPhrase (a the)
open Semantics.Templates.NounWord (creature)
open Semantics.Templates.SliceCount (counted)

namespace Semantics.Cards

def targetPlayerOrPlaneswalker : NounPhrase := (match (Semantics.Cards.«Heart of Bogardan») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons
        (Action.simultaneously (List.cons (Action.instruct _ (Action.dealDamage _ part _)) _)) _)))
        _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okTargetPlayerOrPlaneswalker :
    NounPhrase.checkAt none (DRS.ofList []) targetPlayerOrPlaneswalker = [] := by decide

def targetOpponentOrPlaneswalker : NounPhrase := target (or [isPlaneswalker, isOpponent])
theorem okTargetOpponentOrPlaneswalker :
    NounPhrase.checkAt none (DRS.ofList []) targetOpponentOrPlaneswalker = [] := by decide

/-- Bioplasm -/
def bioplasmAttack : GameEvent := (match (Semantics.Cards.«Bioplasm») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.triggered part _ _ _ _ _ _ _) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okBioplasmAttack : GameEvent.check (DRS.ofList []) bioplasmAttack = [] := by decide
def bioplasmExile : Action := (match (Semantics.Cards.«Bioplasm») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons part _))) _) =>
        some part
      | _ => none
    | _ => none).get (by decide)
theorem okBioplasmExile :
    Action.check ((stepGameEvent 512 (DRS.ofList []) bioplasmAttack).1) bioplasmExile = [] := by
  decide
def bioplasmAfterExile : DRS :=
  (stepAction 512 .postEvent .obj
    ((stepGameEvent 512 (DRS.ofList []) bioplasmAttack).1) bioplasmExile).1

def phyrexianInfiltrator : Action := exchangeControl thisCreature (target isCreature)
theorem okPhyrexianInfiltrator : Action.check (DRS.ofList []) phyrexianInfiltrator = [] := by decide

/-- Vedalken Squirrel-Whacker -/
def vedalkenSquirrelWhackerReroll : Action :=
  replaceEvent (would (Semantics.Subject.actor you) (rollDice anyAmount (.sides 6)))
    (instead (sequentially
      [ rollDice thatMuch .thoseDice,
        may
          (chooseOne
            (exchangeValues (theOutcome .rollResult) (basePowerOf thisCreature))
            (exchangeValues (theOutcome .rollResult) (baseToughnessOf thisCreature))) ]))
    none
theorem okVedalkenSquirrelWhackerReroll :
    Action.check (DRS.ofList []) vedalkenSquirrelWhackerReroll = [] := by decide

/-- "each creature that player or that planeswalker's controller controls" -/
def eachCreatureThatSplitControls : NounPhrase :=
  (match (Semantics.Cards.«Heart of Bogardan») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons
        (Action.simultaneously (List.cons _ (List.cons (Action.instruct _ (Action.dealDamage _ part
        _)) _))) _))) _)) => some part
      | _ => none
    | _ => none).get (by decide)

/-- Boreas Charger's description -/
def opponentWithMoreLands : Predicate :=
  (match (Semantics.Cards.«Land Tax») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.triggered _ _ _ _ _ _ (Option.some (Condition.exists_
        (NounPhrase.described _ part))) _) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okOpponentWithMoreLands :
    Predicate.checkAt .player (DRS.ofList []) opponentWithMoreLands = [] := by decide

def bioplasmCardTest : Condition := (match (Semantics.Cards.«Bioplasm») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons _ (List.cons
        (Action.doIf part _ _ _) _)))) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okBioplasmCardTest :
    Condition.check (nextClause bioplasmAfterExile) bioplasmCardTest = [] := by decide
def bioplasmAfterTest : DRS :=
  (stepCondition 512 (nextClause bioplasmAfterExile) bioplasmCardTest).1

/-- Tahngarth, First Mate -/
def tahngarthHeader : GameEvent := does (Semantics.Subject.actor anOpponent) (attackWith (counted (orMore 1) isCreature))
theorem okTahngarthHeader : GameEvent.check (DRS.ofList []) tahngarthHeader = [] := by decide

def ashnodsBattleGear : Ability := static (mayDeclineUntap thisArtifact you)
theorem okAshnodsBattleGear : Ability.check (DRS.ofList []) ashnodsBattleGear = [] := by decide

/-- Tahngarth, First Mate -/
def tahngarthChoosesDefender : Action :=
  choose (a (and [ or [isPlaneswalker, isPlayer],
               inCombat .attackedBy (that Semantics.Templates.NounWord.player) ]))
theorem okTahngarthChoosesDefender :
    Action.check ((stepGameEvent 512 (DRS.ofList []) tahngarthHeader).1)
      tahngarthChoosesDefender = [] := by
  decide

/-- Thick-Skinned Goblin: "You may pay {0} rather than pay the echo cost for permanents you
control." Echo's cost is paid while its triggered ability resolves [CR#702.30a], neither a
spell's mana cost nor an activation cost, so it is no alternative cost: the payment of echo's
cost (the `keyword` field) is the event a replacement watches, and paying {0} is what may happen
in its place, the shape an "instead" effect takes [CR#614.1a]. Declining the `may` leaves the
echo cost to be paid. -/
def thickSkinnedGoblinEchoPayment : Ability :=
  (match (Semantics.Cards.«Thick-Skinned Goblin») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons part _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okThickSkinnedGoblinEchoPayment :
    Ability.check (DRS.ofList []) thickSkinnedGoblinEchoPayment = [] := by decide

/-- The keyword family terms of a card's Same-Is-True-For lines, in order. -/
def sameIsTrueForFamilies (card : Card) : List KeywordTerm :=
  match card with
  | .singleFaced face => face.characteristics.text.flatMap fun
    | .sameIsTrueFor _ keywords => keywords.filter fun
      | .anyIn _ => Bool.true
      | _ => Bool.false
    | _ => []
  | _ => []
def protectionAbilities : KeywordTerm :=
  ((sameIsTrueForFamilies (Semantics.Cards.«Concerted Effort»))[1]?).get (by decide)
def landwalkAbilities : KeywordTerm :=
  ((sameIsTrueForFamilies (Semantics.Cards.«Cairn Wanderer»))[0]?).get (by decide)
/-- Hexproof and each "hexproof from [quality]" [CR#702.11d]. -/
def hexproofAbilities : KeywordTerm :=
  ((sameIsTrueForFamilies (Semantics.Cards.«Odric, Lunarch Marshal»))[0]?).get (by decide)
/-- Shay Cormac -/
def wardAbilities : KeywordTerm := .anyIn ⟨ward.label, none⟩

/-- Avarice Totem -/
def avariceTotemExchange : Action :=
  exchangeControl thisArtifact (target (and [isPermanent, non isLand]))
theorem okAvariceTotemExchange : Action.check (DRS.ofList []) avariceTotemExchange = [] := by decide

/-- Cloudspire Captain: "This creature saddles Mounts and crews Vehicles as though its power were
2 greater.", indicative as Hotshot Mechanic's is [CR#609.4]. -/
def cloudspireCaptainCrewLine : StaticSpec :=
  conjunction
    [ asThough (Semantics.Subject.actor thisCreature) (saddle (allOf (hasSubtype mount))) (greaterBy .power 2),
      asThough (Semantics.Subject.actor thisCreature) (crew (allOf (hasSubtype vehicle))) (greaterBy .power 2) ]
theorem okCloudspireCaptainCrewLine :
    StaticSpec.check (DRS.ofList []) cloudspireCaptainCrewLine = [] := by
  decide

/-- Soul Conduit -/
def soulConduitExchange : Ability :=
  activated (compound [(Semantics.Cost.mana [generic 6]), tapSymbol])
    (exchangeLifeTotals (targets (exactly 2) isPlayer))
theorem okSoulConduitExchange : Ability.check (DRS.ofList []) soulConduitExchange = [] := by decide

/-- Lightning Bolt -/
def bolt : Action := instruct this_ (dealDamage 3 (target isAnyTarget))
theorem okBolt : Action.check (DRS.ofList []) bolt = [] := by decide

def karplusanYeti : Action :=
  sequentially
    [ dealDamageOwnPower thisCreature (target isCreature),
      dealDamageOwnPower (that creature) thisCreature ]
theorem okKarplusanYeti : Action.check (DRS.ofList []) karplusanYeti = [] := by decide

/-- Inkshield -/
def inkshieldRider : Action :=
  establishThroughout
    (replace (would any (dealDamage anyAmount you (some .combat)))
      (instead
        (preventDamage allDamage
          (also :=
            create preventedThisWay (Semantics.TokenSpec.written (creatureToken 2 1 [.white, .black] [inkling] [flying]))))))
    thisTurn
theorem okInkshieldRider : Action.check (DRS.ofList []) inkshieldRider = [] := by decide

/-- Quenchable Fire -/
def quenchableFire : Action :=
  sequentially
    [ instruct this_ (dealDamage 3 targetPlayerOrPlaneswalker),
      delayUnlessPaid (beginningOf .upkeep you nextTime) (instruct this_ (dealDamage 3 thatJoin))
        splitOverPlaneswalker (Semantics.Cost.mana [pip .blue]) ]
theorem okQuenchableFire : Action.check (DRS.ofList []) quenchableFire = [] := by decide

/-- Garruk Relentless's first 0: "Garruk Relentless deals 3 damage to target creature. That
creature deals damage equal to its power to him." -/
def garrukRelentlessFight : Action :=
  (match (Semantics.Cards.«Garruk Relentless // Garruk, the Veil-Cursed») with
    | Card.transforming face _ => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.activated _ part _ _ _ _) _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okGarrukRelentlessFight : Action.check (DRS.ofList []) garrukRelentlessFight = [] := by
  decide

/-- Harness Infinity: the exchange action alone, as the Idris bench holds it. -/
def harnessInfinityExchange : Action := exchangeZones Semantics.Templates.Location.hand Semantics.Templates.Location.graveyard
theorem okHarnessInfinityExchange : Action.check (DRS.ofList []) harnessInfinityExchange = [] := by
  decide

/-- Boseiju, Who Shelters All -/
def boseijuMana : Ability :=
  activated (compound [tapSymbol, (Semantics.Cost.perform (payLife 2))])
    (addMana 1 (manaOf [[.colorless]])
      [ whenSpentOn .affectsIt Bool.false (a (and [isInstantOrSorcery, isSpell]))
          (establish (cant any (Semantics.Definitions.Action.counter (that .spell)))) ])
theorem okBoseijuMana : Ability.check (DRS.ofList []) boseijuMana = [] := by decide

end Semantics.Cards

end
