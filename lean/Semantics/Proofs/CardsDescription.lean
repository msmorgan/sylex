import Semantics.Cards.Data.Fixtures
import Semantics.Check
import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsHelpers

-- Canon cards are `noncomputable` data (`Semantics.Cards.Data`); so is what reads them.
noncomputable section


/-!
# Semantics.Cards.Description

Port of `idris/src/Experimental/Cards/Description.idr`: the printed cards of the Description
family, and the phrase-level bench items beside them (a noun, predicate, condition,
action, or ability that elaborated in Idris), each paired with an `ok…` theorem that runs
the checker on it at the bindings its Idris type named.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Glyph of Destruction -/
def glyphOfDestruction : Action :=
  get (target (and [isBlocking, wall, youControl]))
    (up 10) (up 0) untilEndOfCombat
theorem okGlyphOfDestruction : Action.check (DRS.ofList []) glyphOfDestruction = [] := by decide

def rawNonattacking : Predicate := and [isCreature, non isAttacking, non isBlocking]
theorem okRawNonattacking : Predicate.checkAt .object (DRS.ofList []) rawNonattacking = [] := by decide

/-- Harmony of Nature -/
def harmonyOfNature : Action :=
  sequentially
    [ tap (counted anyNumber (and [isUntapped, isCreature, youControl])),
      doForEach (each (and [isCreature, was (tap which) thisWay]))
        (gainLife 4) ]
theorem okHarmonyOfNature : Action.check (DRS.ofList []) harmonyOfNature = [] := by decide

/-- Rats of Rath -/
def ratsOfRath : Action :=
  destroy (target (and [or [isArtifact, isCreature, isLand], youControl]))
theorem okRatsOfRath : Action.check (DRS.ofList []) ratsOfRath = [] := by decide

/-- "another creature or land", after a target on the battlefield. -/
def anotherDisjunctPhrase : Predicate := and [or [isCreature, isLand], isOther]
theorem okAnotherDisjunctPhrase :
    Predicate.checkAt .object (DRS.ofList [{ det := .target, plur := .one, payload := .object [] (some .battlefield) none none none }]) anotherDisjunctPhrase = [] := by
  decide

/-- Defeat -/
def defeat : Action :=
  destroy (target (and [isCreature, hasPower .atMost 2]))
theorem okDefeat : Action.check (DRS.ofList []) defeat = [] := by decide

/-- Terashi's Verdict -/
def terashisVerdict : Action :=
  destroy (target (and [isCreature, isAttacking, hasPower .atMost 3]))
theorem okTerashisVerdict : Action.check (DRS.ofList []) terashisVerdict = [] := by decide

/-- Pillar of Light -/
def pillarOfLight : Action :=
  exile (target (and [isCreature, hasToughness .atLeast 4]))
theorem okPillarOfLight : Action.check (DRS.ofList []) pillarOfLight = [] := by decide

/-- Unholy Annex -/
def unholyAnnex : Action :=
  sequentially
    [ draw 1,
      doIf (exists_ (and [demon, youControl]))
        (sequentially [instruct eachOpponent (loseLife 2), gainLife 2])
        (loseLife 2) ]
theorem okUnholyAnnex : Action.check (DRS.ofList []) unholyAnnex = [] := by decide

/-- War Screecher -/
def warScreecher : Action :=
  get (allOf (and [isCreature, youControl, isOther])) (up 1) (up 1)
    untilEndOfTurn
theorem okWarScreecher : Action.check (DRS.ofList []) warScreecher = [] := by decide

/-- Mind Flayer -/
def mindFlayer : Action :=
  gainControl (target isCreature)
    (forAsLongAs (is thisCreature youControl))
theorem okMindFlayer : Action.check (DRS.ofList []) mindFlayer = [] := by decide

/-- Arc Lightning -/
def arcLightning : Action :=
  dealDivided this_ 3 (targets (oneThrough 3) isAnyTarget)
theorem okArcLightning : Action.check (DRS.ofList []) arcLightning = [] := by decide

/-- Boulderfall -/
def boulderfall : Action :=
  dealDivided this_ 5 (targets anyNumber isAnyTarget)
theorem okBoulderfall : Action.check (DRS.ofList []) boulderfall = [] := by decide

/-- Banisher Priest -/
def banisherPriest : Action :=
  exileUntil (target (and [isCreature, anOpponentControls]))
    (does thisCreature leave)
theorem okBanisherPriest : Action.check (DRS.ofList []) banisherPriest = [] := by decide

/-- Tezzeret, Artifice Master -/
def tezzeretDrawTwo : Action :=
  replace (draw 1)
    (doIf (compareAmt (countOf (and [isArtifact, youControl]))
            .atLeast 3)
      (instead (draw 2)))
theorem okTezzeretDrawTwo : Action.check (DRS.ofList []) tezzeretDrawTwo = [] := by decide

/-- Zimone, Quandrix Prodigy -/
def zimoneDrawTwo : Action :=
  replace (draw 1)
    (doIf (compareAmt (countOf (and [isLand, youControl])) .atLeast 8)
      (instead (draw 2)))
theorem okZimoneDrawTwo : Action.check (DRS.ofList []) zimoneDrawTwo = [] := by decide

/-- Terrifying Presence -/
def terrifyingPresenceAnchor : Predicate := and [isCreature, otherThan (target isCreature)]
theorem okTerrifyingPresenceAnchor : Predicate.checkAt .object (DRS.ofList []) terrifyingPresenceAnchor = [] := by
  decide

/-- Survival Cache -/
def survivalCache : Action :=
  sequentially
    [ gainLife 2,
      doIf (compareAmt (lifeTotalOf you) .greater (lifeTotalOf anOpponent))
        (draw 1) ]
theorem okSurvivalCache : Action.check (DRS.ofList []) survivalCache = [] := by decide

/-- "each opponent with three or more poison counters" -/
def corruptedOpponents : NounPhrase :=
  each (and [isOpponent, compare .atLeast 3 (axes := [.counter poison])])
theorem okCorruptedOpponents : NounPhrase.checkAt (some .player) (DRS.ofList []) corruptedOpponents = [] := by
  decide

/-- War Tax -/
def warTaxScaledPayment : Action :=
  establishThroughout (gatedBy (scaledGenericMana
        (forEach (letter .x) (and [isCreature, isAttacking,
            controlledBy they]))) (allOf isCreature) attack) thisTurn
theorem okWarTaxScaledPayment : Action.check (DRS.ofList [letterB .x]) warTaxScaledPayment = [] := by
  decide

/-- Croaking Counterpart -/
def croakingCounterpartCopy : Action :=
  create 1
    (copyOf (target (and [isCreature, non frog]))
      (copyCharacteristics {
                 colors := [.green], subtypes := [frog], power := stat 1,
                 toughness := stat 1 } false))
    []
theorem okCroakingCounterpartCopy : Action.check (DRS.ofList []) croakingCounterpartCopy = [] := by decide

/-- "a loyalty ability of enchanted planeswalker" -/
def loyaltyAbilityOfEnchanted : NounPhrase :=
  (match (cardNamed "Elspeth's Talent").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons (Ability.triggered (GameEvent.does _ (Action.ownScope _
        (Action.procedure _ (NounPhrase.firstMention _ (NounPhrase.inCaller part)) _ _ _ _))) _ _ _
        _ _ _ _) _))) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okLoyaltyAbilityOfEnchanted :
    NounPhrase.checkAt (some .object) (DRS.ofList []) loyaltyAbilityOfEnchanted = [] := by
  decide

/-- Balance of Power -/
def balanceOfPower : Action :=
  doIf (compareAmt (countOf (cardIn (handOf (target isOpponent)))) .greater
          (countOf (cardIn yourHand)))
    (draw theDifference)
theorem okBalanceOfPower : Action.check (DRS.ofList []) balanceOfPower = [] := by decide

/-- Spark Fiend -/
def sparkFiendUpkeepRoll : Action :=
  sequentially
    [ rollDice 2 (.sides 6),
      doIf (compareAmt (theOutcome .rollResult) .eq 7) (sacrifice thisCreature) ]
theorem okSparkFiendUpkeepRoll : Action.check (DRS.ofList []) sparkFiendUpkeepRoll = [] := by decide

/-- Erebos, God of the Dead -/
def devotionCondition : Condition := compareAmt (devotion you (color .black)) .less 5
theorem okDevotionCondition : Condition.check (DRS.ofList []) devotionCondition = [] := by decide

/-- Multiple Choice -/
def multipleChoiceFirstArm : Action :=
  doIf (compareAmt (letter .x) .eq 1)
    (sequentially [scry 1, draw 1])
theorem okMultipleChoiceFirstArm : Action.check (DRS.ofList []) multipleChoiceFirstArm = [] := by decide

/-- Multiple Choice, fourth arm -/
def multipleChoiceFourthGate : Condition := compareAmt (letter .x) .atLeast 4
theorem okMultipleChoiceFourthGate : Condition.check (DRS.ofList []) multipleChoiceFourthGate = [] := by decide

/-- Fell the Mighty -/
def fellTheMighty : Action :=
  destroy
    (allOf (and [isCreature,
                  hasPower .greater (powerOf (target isCreature))]))
theorem okFellTheMighty : Action.check (DRS.ofList []) fellTheMighty = [] := by decide

/-- Baleful Mastery's paid read -/
def balefulMasteryPaidRead : Ability :=
  spell (doIf (costWasPaid .theAlternative none this_) (instruct anOpponent (draw 1)))
theorem okBalefulMasteryPaidRead : Ability.check (DRS.ofList []) balefulMasteryPaidRead = [] := by decide

/-- Karai, Future of the Foot -/
def karaiSneakPaidThisTurn : Amount := paidCostRead (.byKeyword sneak) thisTurn this_
theorem okKaraiSneakPaidThisTurn : Amount.check (DRS.ofList []) karaiSneakPaidThisTurn = [] := by decide

/-- Requiting Hex's read -/
def requitingHexAdditionalRead : Ability :=
  spell (doIf (costWasPaid .theAdditional none this_) (gainLife 2))
theorem okRequitingHexAdditionalRead : Ability.check (DRS.ofList []) requitingHexAdditionalRead = [] := by
  decide

def bioplasmExiledCard : NounPhrase := the (and [was (exile which), isCard])
theorem okBioplasmExiledCard :
    NounPhrase.checkAt (some .object) bioplasmAfterExile bioplasmExiledCard = [] := by decide

/-- "the top card of each player's library" -/
def playersTopCardSlice : NounPhrase := slice .top 1 (libraryOf eachPlayer)
theorem okPlayersTopCardSlice : NounPhrase.checkAt (some .object) (DRS.ofList []) playersTopCardSlice = [] := by
  decide

/-- Deepglow Skate's recipient -/
def deepglowSkateRecipient : NounPhrase := targets anyNumber isPermanent
theorem deepglowSkateRecipientRefused : deepglowSkateRecipient.perMemberOk = false := by decide

/-- Weftwalking -/
def weftwalkingShuffle : Action :=
  sequentially
    [ shuffleInto (both (allOf (cardIn yourHand)) (allOf (cardIn yourGraveyard))),
      draw 7 ]
theorem okWeftwalkingShuffle : Action.check (DRS.ofList []) weftwalkingShuffle = [] := by decide

/-- Gaea's Revenge's protection-shaped phrase -/
def nongreenSpellsOrAbilities : Predicate :=
  or [ and [isSpell, non (hasColor .green)],
        and [ abilityHead .anyOnStack,
               abilityOf (a (and [isSource, non (hasColor .green)])) ] ]
theorem okNongreenSpellsOrAbilities :
    Predicate.checkAt .object (DRS.ofList []) nongreenSpellsOrAbilities = [] := by decide

/-- Hot Pursuit -/
def twoOrMorePlayersHaveLost : Condition :=
  compareAmt (countOf (and
    [isPlayer, did which loseGame thisGame])) .atLeast 2
theorem okTwoOrMorePlayersHaveLost : Condition.check (DRS.ofList []) twoOrMorePlayersHaveLost = [] := by decide

def commanderCreaturesYouOwn : Predicate :=
  and [isCreature, hasDesignation commander, youOwn]
theorem okCommanderCreaturesYouOwn : Predicate.checkAt .object (DRS.ofList []) commanderCreaturesYouOwn = [] := by
  decide

/-- Idol of Endurance -/
def creatureSpellFromAmongExiled : NounPhrase :=
  fromAmong (exactly 1) (and [isCreature, isSpell]) (allOf (cardIn (exiledWith thisArtifact)))
theorem okCreatureSpellFromAmongExiled :
    NounPhrase.checkAt (some .object) (DRS.ofList []) creatureSpellFromAmongExiled = [] := by decide

def ringHasTemptedYouTwiceThisGame : Condition :=
  compareAmt (eventCount theRingTemptsYou you thisGame)
    .atLeast 2
theorem okRingHasTemptedYouTwiceThisGame :
    Condition.check (DRS.ofList []) ringHasTemptedYouTwiceThisGame = [] := by decide

def creatureCardAnywhere : Predicate := and [isCreature, isCard]
theorem okCreatureCardAnywhere : Predicate.checkAt .object (DRS.ofList []) creatureCardAnywhere = [] := by decide
def permanentCardAnywhere : Predicate := isPermanentCard
theorem okPermanentCardAnywhere : Predicate.checkAt .object (DRS.ofList []) permanentCardAnywhere = [] := by decide

/-- "target card on the stack" -/
def targetCardOnTheStack : NounPhrase := target cardOnTheStack
theorem okTargetCardOnTheStack : NounPhrase.checkAt (some .object) (DRS.ofList []) targetCardOnTheStack = [] := by
  decide
/-- "each token on the battlefield" -/
def eachTokenOnTheBattlefield : NounPhrase := allOf tokenOnTheBattlefield
theorem okEachTokenOnTheBattlefield :
    NounPhrase.checkAt (some .object) (DRS.ofList []) eachTokenOnTheBattlefield = [] := by decide
/-- "each emblem you own" -/
def eachEmblemYouOwn : NounPhrase := allOf (and [isEmblem, youOwn])
theorem okEachEmblemYouOwn : NounPhrase.checkAt (some .object) (DRS.ofList []) eachEmblemYouOwn = [] := by decide
/-- "a copy of a card" -/
def aCopyOfACard : NounPhrase := a isCopyOfACard
theorem okACopyOfACard : NounPhrase.checkAt (some .object) (DRS.ofList []) aCopyOfACard = [] := by decide

/-- Keeper of the Flame, Keeper of the Light -/
def opponentWithMoreLifeThanYou : Predicate :=
  and [isOpponent, compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal])]
theorem okOpponentWithMoreLifeThanYou :
    Predicate.checkAt .player (DRS.ofList []) opponentWithMoreLifeThanYou = [] := by decide
/-- Namor, Atlantean King -/
def playerWithMoreLifeThanYou : Predicate :=
  and [isPlayer, compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal])]
theorem okPlayerWithMoreLifeThanYou :
    Predicate.checkAt .player (DRS.ofList []) playerWithMoreLifeThanYou = [] := by decide
def noOpponentHasMoreLifeThanYou : Condition :=
  not (exists_
         (and [isOpponent, compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal])]))
theorem okNoOpponentHasMoreLifeThanYou :
    Condition.check (DRS.ofList []) noOpponentHasMoreLifeThanYou = [] := by decide
def someOpponentLacksMoreLifeThanYou : Condition :=
  exists_ (and [isOpponent,
                 not (compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal]))])
theorem okSomeOpponentLacksMoreLifeThanYou :
    Condition.check (DRS.ofList []) someOpponentLacksMoreLifeThanYou = [] := by decide
def yourOpponentsHaveMoreLifeThanYou : Condition :=
  is yourOpponents
    (compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal]))
theorem okYourOpponentsHaveMoreLifeThanYou :
    Condition.check (DRS.ofList []) yourOpponentsHaveMoreLifeThanYou = [] := by decide

def rawExile : Action := exile (target isCreature)
theorem okRawExile : Action.check (DRS.ofList []) rawExile = [] := by decide
def disenchant : Action := destroy (target (or [isArtifact, isEnchantment]))
theorem okDisenchant : Action.check (DRS.ofList []) disenchant = [] := by decide
def icyManipulator : Action := tap (target (or [isArtifact, isCreature, isLand]))
theorem okIcyManipulator : Action.check (DRS.ofList []) icyManipulator = [] := by decide
def divination : Action := draw 2
theorem okDivination : Action.check (DRS.ofList []) divination = [] := by decide
def ancestralRecall : Action := instruct (target isPlayer) (draw 3)
theorem okAncestralRecall : Action.check (DRS.ofList []) ancestralRecall = [] := by decide
def lifeTotalBecomesOne : Action := instruct (target isPlayer) (setLife 1)
theorem okLifeTotalBecomesOne : Action.check (DRS.ofList []) lifeTotalBecomesOne = [] := by decide

/-- Berserker's Frenzy's roll -/
def berserkersFrenzyRoll : Action :=
  sequentially [rollDice 2 (.sides 20), ignoreOutcomes (extreme .lowest)]
theorem okBerserkersFrenzyRoll : Action.check (DRS.ofList []) berserkersFrenzyRoll = [] := by decide
def ironMastiffIgnore : Action :=
  sequentially
    [ rollDice
        (countOf (and [isPlayer, inCombat .attackedBy (combatPlayer .attacking)]))
        (.sides 20),
      ignoreOutcomes (allBut .highest) ]
theorem okIronMastiffIgnore : Action.check (DRS.ofList []) ironMastiffIgnore = [] := by decide

/-- Xenosquirrels -/
def xenosquirrelsShift : Ability :=
  after (does you (rollDice 1 .anyDie)) (may
      (removeCounters (exactly 1) p1p1Counter thisCreature)
      (shiftResult 1))
theorem okXenosquirrelsShift : Ability.check (DRS.ofList []) xenosquirrelsShift = [] := by decide

theorem playersTopCardIsPlural : playersTopCardSlice.plur = .many := by decide

/-- Aetheric Amplifier -/
def doubleYourOwnCounters : Action := doubleCounters you
theorem okDoubleYourOwnCounters : Action.check (DRS.ofList []) doubleYourOwnCounters = [] := by decide

def permanentCardPhrase : Predicate := isPermanentCard
theorem permanentCardIsPlaceless : permanentCardPhrase.phraseZone .object = none := by decide

/-- Archpriest of Iona -/
def archpriestOfIonaPower : Ability := static (ptDefinition thisCreature .powerAlone partySize)
theorem okArchpriestOfIonaPower : Ability.check (DRS.ofList []) archpriestOfIonaPower = [] := by decide
def archpriestOfIonaFullParty : Ability :=
  at_ (beginningOfYour .combat) (if_ := fullParty)
    (establishFor (target isCreature)
      [getsPt it (up 1) (up 1), abilityGrant it flying]
      untilEndOfTurn)
theorem okArchpriestOfIonaFullParty : Ability.check (DRS.ofList []) archpriestOfIonaFullParty = [] := by decide

/-- Squad Commander -/
def squadCommanderTokens : Ability :=
  when (does thisCreature enter)
    (create partySize (creatureToken 1 1 [.white] [kor, warrior]))
theorem okSquadCommanderTokens : Ability.check (DRS.ofList []) squadCommanderTokens = [] := by decide

/-- At Knifepoint -/
def atKnifepointOutlaws : Ability :=
  (match (cardNamed "At Knifepoint").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons part _) => some part
      | _ => none
    | _ => none).get (by decide)
def atKnifepointCrime : Ability :=
  (match (cardNamed "At Knifepoint").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons part _)) => some part
      | _ => none
    | _ => none).get (by decide)

end Semantics.Cards

end
