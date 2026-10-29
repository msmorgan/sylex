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

namespace Semantics.Cards

def targetPlayerOrPlaneswalker : NounPhrase := (match (cardNamed "Heart of Bogardan").card with
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
def bioplasmAttack : GameEvent := (match (cardNamed "Bioplasm").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.triggered part _ _ _ _ _ _ _) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okBioplasmAttack : GameEvent.check (DRS.ofList []) bioplasmAttack = [] := by decide
def bioplasmExile : Action := (match (cardNamed "Bioplasm").card with
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
  replaceEvent (would you (rollDice anyAmount (.sides 6)))
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
  (match (cardNamed "Heart of Bogardan").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons
        (Action.simultaneously (List.cons _ (List.cons (Action.instruct _ (Action.dealDamage _ part
        _)) _))) _))) _)) => some part
      | _ => none
    | _ => none).get (by decide)

/-- Boreas Charger's description -/
def opponentWithMoreLands : Predicate :=
  (match (cardNamed "Land Tax").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.triggered _ _ _ _ _ _ (Option.some (Condition.exists_
        (NounPhrase.described _ part))) _) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okOpponentWithMoreLands :
    Predicate.checkAt .player (DRS.ofList []) opponentWithMoreLands = [] := by decide

def bioplasmCardTest : Condition := (match (cardNamed "Bioplasm").card with
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
def tahngarthHeader : GameEvent := does anOpponent (attackWith (counted (orMore 1) isCreature))
theorem okTahngarthHeader : GameEvent.check (DRS.ofList []) tahngarthHeader = [] := by decide

def ashnodsBattleGear : Ability := static (mayDeclineUntap thisArtifact you)
theorem okAshnodsBattleGear : Ability.check (DRS.ofList []) ashnodsBattleGear = [] := by decide

/-- Tahngarth, First Mate -/
def tahngarthChoosesDefender : Action :=
  choose (a (and [ or [isPlaneswalker, isPlayer],
               inCombat .attackedBy (that player) ]))
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
  (match (cardNamed "Thick-Skinned Goblin").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons part _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okThickSkinnedGoblinEchoPayment :
    Ability.check (DRS.ofList []) thickSkinnedGoblinEchoPayment = [] := by decide

def protectionAbilities : KeywordTerm := (match (cardNamed "Concerted Effort").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons _ (List.cons _
        (List.cons _ (List.cons _ (List.cons _ (List.cons (Action.doIf (Condition.exists_
        (NounPhrase.described _ (Predicate.coord _ (List.cons _ (List.cons _ (List.cons
        (Predicate.hasKeyword part) _)))))) _ _ _) _)))))))) _) => some part
      | _ => none
    | _ => none).get (by decide)
def landwalkAbilities : KeywordTerm := (match (cardNamed "Cairn Wanderer").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.static (StaticSpec.conjunction _ (List.cons _ (List.cons _
        (List.cons _ (List.cons _ (List.cons _ (List.cons _ (List.cons (StaticSpec.conditional
        (StaticSpec.characteristicChange _ (List.cons (CharacteristicEdit.addedKeywordTerm part _)
        _)) _ _) _))))))))) _))
        => some part
      | _ => none
    | _ => none).get (by decide)
/-- Hexproof and each "hexproof from [quality]" [CR#702.11d]. -/
def hexproofAbilities : KeywordTerm := (match (cardNamed "Odric, Lunarch Marshal").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons _ (List.cons _
        (List.cons _ (List.cons _ (List.cons _ (List.cons (Action.doIf (Condition.exists_
        (NounPhrase.described _ (Predicate.coord _ (List.cons _ (List.cons _ (List.cons
        (Predicate.hasKeyword part) _)))))) _ _ _) _)))))))) _) => some part
      | _ => none
    | _ => none).get (by decide)
/-- Shay Cormac -/
def wardAbilities : KeywordTerm := .anyIn ⟨ward, none⟩

/-- Avarice Totem -/
def avariceTotemExchange : Action :=
  exchangeControl thisArtifact (target (and [isPermanent, non isLand]))
theorem okAvariceTotemExchange : Action.check (DRS.ofList []) avariceTotemExchange = [] := by decide

/-- Cloudspire Captain: "This creature saddles Mounts and crews Vehicles as though its power were
2 greater.", indicative as Hotshot Mechanic's is [CR#609.4]. -/
def cloudspireCaptainCrewLine : StaticSpec :=
  conjunction
    [ asThough thisCreature (saddle (allOf (hasSubtype mount))) (greaterBy .power 2),
      asThough thisCreature (crew (allOf (hasSubtype vehicle))) (greaterBy .power 2) ]
theorem okCloudspireCaptainCrewLine :
    StaticSpec.check (DRS.ofList []) cloudspireCaptainCrewLine = [] := by
  decide

/-- Soul Conduit -/
def soulConduitExchange : Ability :=
  activated (compound [[generic 6], tapSymbol])
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
            create preventedThisWay (creatureToken 2 1 [.white, .black] [inkling] [flying])))))
    thisTurn
theorem okInkshieldRider : Action.check (DRS.ofList []) inkshieldRider = [] := by decide

/-- Quenchable Fire -/
def quenchableFire : Action :=
  sequentially
    [ instruct this_ (dealDamage 3 targetPlayerOrPlaneswalker),
      delayUnlessPaid (beginningOf .upkeep you nextTime) (instruct this_ (dealDamage 3 thatJoin))
        splitOverPlaneswalker [pip .blue] ]
theorem okQuenchableFire : Action.check (DRS.ofList []) quenchableFire = [] := by decide

/-- Garruk Relentless's first 0: "Garruk Relentless deals 3 damage to target creature. That
creature deals damage equal to its power to him." -/
def garrukRelentlessFight : Action :=
  (match (cardNamed "Garruk Relentless // Garruk, the Veil-Cursed").card with
    | Card.transforming face _ => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.activated _ part _ _ _ _) _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okGarrukRelentlessFight : Action.check (DRS.ofList []) garrukRelentlessFight = [] := by
  decide

/-- Harness Infinity: the exchange action alone, as the Idris bench holds it. -/
def harnessInfinityExchange : Action := exchangeZones hand graveyard
theorem okHarnessInfinityExchange : Action.check (DRS.ofList []) harnessInfinityExchange = [] := by
  decide

/-- Boseiju, Who Shelters All -/
def boseijuMana : Ability :=
  activated (compound [tapSymbol, payLife 2])
    (addMana 1 (manaOf [[.colorless]])
      [ whenSpentOn .affectsIt false (a (and [isInstantOrSorcery, isSpell]))
          (establish (cant any (counter (that .spell)))) ])
theorem okBoseijuMana : Ability.check (DRS.ofList []) boseijuMana = [] := by decide

end Semantics.Cards

end
