import Semantics.Cards.Data.Fixtures
import Semantics.Check
import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsHelpers

-- Canon cards are `noncomputable` data (`Semantics.Cards.Data`); so is what reads them.
noncomputable section


/-!
# Semantics.Cards.Choice

Port of `idris/src/Experimental/Cards/Choice.idr`: the printed cards of the Choice family
(choices, modes, may, permissions to play, chosen qualities) and the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (a each counted the)
open Semantics.Templates.Predicate (and or not was quality attachedTo)
open Semantics.Templates.ManaSymbol (generic)
open Semantics.Templates.Location (and or not)
open Semantics.Templates.NounPhrase (and or)
open Semantics.Templates.Timing (not)
open Semantics.Templates.Replacement (doIf)

open Semantics.Definitions.CardType (creature)
open Semantics.Templates.Action (doIf target)
open Semantics.Templates.Condition (and not or was)
open Semantics.Templates.CostShift (generic)
open Semantics.Templates.DetPhrase (a each the)
open Semantics.Templates.MoveRider (attachedTo)
open Semantics.Templates.NounWord (creature)
open Semantics.Templates.QualityPayload (quality)
open Semantics.Templates.SliceCount (counted)

namespace Semantics.Cards

def carefulStudy : Action :=
  sequentially [draw 2, discard (counted (exactly 2) (cardIn yourHand))]
theorem okCarefulStudy : Action.check (DRS.ofList []) carefulStudy = [] := by decide

def zombieInfestation : Ability :=
  activated ((Semantics.Cost.perform (discard (counted (exactly 2) (cardIn yourHand)))))
    (create 1 (Semantics.TokenSpec.written ((creatureToken 2 2 [.black] [zombie]))))
theorem okZombieInfestation : Ability.check (DRS.ofList []) zombieInfestation = [] := by decide

def fulgentDistraction : Action :=
  sequentially
    [ choose (targets (exactly 2) isCreature),
      tap (those creature),
      unattach (allOf (and [ (Semantics.Predicate.hasSubtype (equipment)),
                               attachedTo (those creature) ])) ]
theorem okFulgentDistraction : Action.check (DRS.ofList []) fulgentDistraction = [] := by decide

/-- "Choose target creature you control and target creature you don't control. … those
creatures": the fight-and-bite opening (Blizzard Brawl, Joust, Tail Swipe, Malamet Battle Glyph).
The targets are chosen as the spell is cast [CR#601.2c]; "those creatures" reads both. -/
def chooseCreatureYouControlAndCreatureYouDont : Action :=
  sequentially
    [ choose (both (target (and [isCreature, youControl]))
                   (target (and [isCreature, youDontControl]))),
      tap (those creature) ]
theorem okChooseCreatureYouControlAndCreatureYouDont :
    Action.check (DRS.ofList []) chooseCreatureYouControlAndCreatureYouDont = [] := by decide

/-- The opening the fight-and-bite cards share: "Choose target creature you control and target
creature you don't control." -/
def chooseFightPair : Action :=
  (match (Semantics.Cards.«Joust») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ (Action.sequentially (List.cons part _))) _) => some part
      | _ => none
    | _ => none).get (by decide)

/-- Joust's postposed "if it's a Knight" reads the one creature its clause re-mentions, the
target you control, and not the target you don't control, which its clause does not mention.
The objects resolved are counted: "you", re-mentioned by each "you control", is resolved too
(`mentionYou`). -/
theorem joustConditionReadsTheClauseSubject :
    let out := (stepAction 16 .postEvent .obj (DRS.ofList []) (sequentially
      [ chooseFightPair,
        doOnlyIf (get (the (and [isCreature, youControl])) (up 2) (up 1) untilEndOfTurn)
          (is it (hasSubtype knight)) ])).1
    ((out.resolvedIds.filterMap out.findById?).filter (·.kind == .object)).map
      (·.controlledByYou) = [some true] := by
  decide

/-- "If it's a Knight, the creature you control gets +2/+1 until end of turn.": a preposed "if"
is read before its clause, so its "it" reads neither target over the other, and is refused. -/
theorem badPreposedConditionReadsNoLaterClauseSubject :
    Action.check (DRS.ofList []) (sequentially
      [ chooseFightPair,
        doIf (is it (hasSubtype knight))
          (get (the (and [isCreature, youControl])) (up 2) (up 1) untilEndOfTurn) ]) =
      [.anaphor .bare .one 2] := by decide

/-- Stolen Uniform, through its third sentence: "Choose target creature you control and target
Equipment. Gain control of that Equipment until end of turn. Attach it to the chosen creature."
"The chosen creature" reads back the creature target, chosen as the spell is cast [CR#601.2c].
Its last sentence, "When you lose control of that Equipment this turn, if it's attached to a
creature you control, unattach it", waits on `syl-lose-control-event`. -/
def stolenUniformOpening : Action :=
  sequentially
    [ choose (both (target (and [isCreature, youControl])) (target (hasSubtype equipment))),
      gainControl (that (.subtype equipment)) untilEndOfTurn,
      attachTo it (the (and [isCreature, isChosen])) ]
theorem okStolenUniformOpening : Action.check (DRS.ofList []) stolenUniformOpening = [] := by
  decide

def continueSpell : Action :=
  sequentially
    [ choose (targets (upTo 4) (and [isCreature, in_ yourGraveyard])),
      returnToBattlefield them ]
theorem okContinueSpell : Action.check (DRS.ofList []) continueSpell = [] := by decide

def kindredDominance : Action :=
  sequentially
    [ choose (a (quality (.subtype .creature))),
      destroy (allOf (and [isCreature, not (ofChosen (.subtype .creature))])) ]
theorem okKindredDominance : Action.check (DRS.ofList []) kindredDominance = [] := by decide

def phantomBlade : Ability :=
  when (does (Semantics.Subject.actor (thisEquipment)) enter)
    (sequentially
      [ attachTo it
          (targets (upTo 1) (and [isCreature, youControl])),
        destroy (targets (upTo 1) (and [isCreature, isOther])) ])
theorem okPhantomBlade : Ability.check (DRS.ofList []) phantomBlade = [] := by decide

/-- Braids's Frightful Return -/
def braidsFrightfulReturn : Action :=
  may (sacrifice (a isCreature))
                                    (instruct eachOpponent (discard (a (cardIn hand))))
theorem okBraidsFrightfulReturn : Action.check (DRS.ofList []) braidsFrightfulReturn = [] := by decide
/-- Daretti, Ingenious Iconoclast -/
def darettisMinusOne : Action :=
  may (sacrifice (a isArtifact)) (destroy (target (or [isArtifact,
      isCreature])))
theorem okDarettisMinusOne : Action.check (DRS.ofList []) darettisMinusOne = [] := by decide

def rainOfThorns : Action :=
  chooseModes (orMore 1)
    [{ action := destroy (target isArtifact) }, { action := destroy (target isEnchantment) }, { action := destroy (target isLand) }]
theorem okRainOfThorns : Action.check (DRS.ofList []) rainOfThorns = [] := by decide

def rankleMasterOfPranks : Action :=
  chooseModes anyNumber
    [ { action := instruct eachPlayer (discard (a (cardIn hand))) },
      { action := sequentially [instruct eachPlayer (loseLife 1), instruct (those player)
                      (draw 1)] },
      { action := instruct eachPlayer (sacrifice (aTheirChoice isCreature)) } ]
theorem okRankleMasterOfPranks : Action.check (DRS.ofList []) rankleMasterOfPranks = [] := by decide

def myrkulsEdict : Action :=
  sequentially [choose anOpponent, instruct (that player) (sacrifice (aTheirChoice isCreature))]
theorem okMyrkulsEdict : Action.check (DRS.ofList []) myrkulsEdict = [] := by decide
def moltingHarpy : Action := doUnless (sacrifice thisCreature) (Semantics.Cost.mana ([generic
    2]))
theorem okMoltingHarpy : Action.check (DRS.ofList []) moltingHarpy = [] := by decide
def carnophage : Action := doUnless (tap thisCreature) (Semantics.Cost.perform ((payLife 1)))
theorem okCarnophage : Action.check (DRS.ofList []) carnophage = [] := by decide
def solitaryConfinement : Action :=
  mayOrElse (discard (a (cardIn yourHand))) (sacrifice thisEnchantment)
theorem okSolitaryConfinement : Action.check (DRS.ofList []) solitaryConfinement = [] := by decide

def yasminKhan : Ability :=
  activated tapSymbol
    (sequentially
      [ exile (topSlice 1),
        establishThroughout (can (Semantics.Subject.actor (you)) (play it)) untilYourNextEndStep ])
theorem okYasminKhan : Ability.check (DRS.ofList []) yasminKhan = [] := by decide

/-- Brazen Cannonade -/
def brazenCannonadePermission : Action :=
  sequentially
    [ exile (topSlice 1),
      establishThroughout (can (Semantics.Subject.actor (you)) (play it)) (untilEvent
                                                         (endOf .combat you nextTime)) ]
theorem okBrazenCannonadePermission : Action.check (DRS.ofList []) brazenCannonadePermission = [] := by
  decide

def thousandMoonsCrackshot : Ability :=
  whenever (does (Semantics.Subject.actor (thisCreature)) attack)
    (offerWhen (pay (Semantics.Cost.mana ([generic 2, pip .white])) .once)
      (tap (target isCreature)))
theorem okThousandMoonsCrackshot : Ability.check (DRS.ofList []) thousandMoonsCrackshot = [] := by decide

def zimoneQuandrixProdigy : Ability :=
  activated (compound [(Semantics.Cost.mana ([generic 1])), tapSymbol])
    (may (putOntoBattlefieldTapped (a (and [isLand, in_ yourHand]))))
theorem okZimoneQuandrixProdigy : Ability.check (DRS.ofList []) zimoneQuandrixProdigy = [] := by decide

def preeminentCaptain : Ability :=
  whenever (does (Semantics.Subject.actor (thisCreature)) attack)
    (may (putOntoBattlefieldTappedAttacking
      (a (and [(Semantics.Predicate.hasSubtype (soldier)), isCreature, in_ yourHand]))))
theorem okPreeminentCaptain : Ability.check (DRS.ofList []) preeminentCaptain = [] := by decide

def skaabRuinator : Ability :=
  static (can (Semantics.Subject.actor (you)) (cast this_ (from_ := yourGraveyard)))
theorem okSkaabRuinator : Ability.check (DRS.ofList []) skaabRuinator = [] := by decide

def escapeToTheWilds : Action :=
  sequentially
    [ exile (topSlice 5),
      establishThroughout (can (Semantics.Subject.actor (you))
                             (play (bare (and [isCard, was (exile which) thisWay]))))
        (untilEvent (endOf .turn you nextTime)) ]
theorem okEscapeToTheWilds : Action.check (DRS.ofList []) escapeToTheWilds = [] := by decide

/-- Muse Vessel -/
def museVesselPlay : Ability :=
  activated (Semantics.Cost.mana ([generic 1]))
    (sequentially
      [ choose (a (cardIn (exiledWith thisArtifact))),
        establishThroughout (can (Semantics.Subject.actor (you)) (play (that card))) thisTurn ])
theorem okMuseVesselPlay : Ability.check (DRS.ofList []) museVesselPlay = [] := by decide

def demonicConsultationChoice : Action := choose (a (quality .cardName))
theorem okDemonicConsultationChoice : Action.check (DRS.ofList []) demonicConsultationChoice = [] := by
  decide
def voidChoice : Action := choose (a (quality .number))
theorem okVoidChoice : Action.check (DRS.ofList []) voidChoice = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngesterPump : Ability :=
  (match (Semantics.Cards.«Phyrexian Ingester») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons part _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okPhyrexianIngesterPump : Ability.check (DRS.ofList []) phyrexianIngesterPump = [] := by decide

/-- Koh, the Face Stealer -/
def kohChooser : Ability :=
  activated (Semantics.Cost.perform ((payLife 1))) (choose (a (and [isCreature, in_ (exiledWith this_)])))
theorem okKohChooser : Ability.check (DRS.ofList []) kohChooser = [] := by decide
/-- Forgotten Lore -/
def forgottenLoreChoice : Action :=
  instruct (target isOpponent) (choose (a (cardIn yourGraveyard)))
theorem okForgottenLoreChoice : Action.check (DRS.ofList []) forgottenLoreChoice = [] := by decide

/-- Psychic Paper minus its three-way coordination -/
def psychicPaperChoiceAndReads : List Ability :=
  [ static (conjunction [ attachChoosing thisEquipment .cardName,
        attachChoosing thisEquipment (.subtype .creature) ]),
    static (conjunction [ qualityChange (equipped creature) .sets
          (chosenQuality (ofTheLastChosen .cardName)),
        qualityChange (equipped creature) .sets
          (chosenQuality (ofTheLastChosen (.subtype .creature))) ]) ]
theorem okPsychicPaperChoiceAndReads : Ability.checkText (DRS.ofList []) psychicPaperChoiceAndReads = [] := by
  decide

def necromentiaChoice : Action :=
  choose (a (qualityFrom .cardName (nameOfCard (not (and [hasSupertype .basic, isLand])))))
theorem okNecromentiaChoice : Action.check (DRS.ofList []) necromentiaChoice = [] := by decide

/-- "play lands from the top of your library" -/
def playLandsFromTop : StaticSpec :=
  (match (Semantics.Cards.«Oracle of Mul Daya») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons (Ability.static part) _))) => some part
      | _ => none
    | _ => none).get (by decide)
/-- Future Sight; Magus of the Future -/
def playLandsAndCastSpellsFromTop : StaticSpec :=
  (match (Semantics.Cards.«Magus of the Future») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.static part) _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okPlayLandsAndCastSpellsFromTop :
    StaticSpec.check (DRS.ofList []) playLandsAndCastSpellsFromTop = [] := by decide
/-- Assemble the Players -/
def castSmallCreatureFromTopOnceEachTurn : StaticSpec :=
  (match (Semantics.Cards.«Assemble the Players») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.static part) _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okCastSmallCreatureFromTopOnceEachTurn :
    StaticSpec.check (DRS.ofList []) castSmallCreatureFromTopOnceEachTurn = [] := by decide

/-- Forgotten Lore -/
def forgottenLoreRepeat : Action :=
  sequentially
    [ instruct (target isOpponent) (choose (a (cardIn yourGraveyard))),
      may (pay (Semantics.Cost.mana ([pip .green])) .once) (repeat_
          againExcludingChosen) ]
theorem okForgottenLoreRepeat : Action.check (DRS.ofList []) forgottenLoreRepeat = [] := by decide

def killingWave : Action :=
  doForEach (each isCreature)
    (instruct (controllerOf it) (doUnless (instruct they (sacrifice it)) (Semantics.Cost.perform ((payLife (letter .x))))))
theorem okKillingWave : Action.check (DRS.ofList []) killingWave = [] := by decide
def fadeAway : Action :=
  doForEach (each isCreature)
    (instruct (controllerOf it) (doUnless (instruct they (sacrifice (a isPermanent))) (Semantics.Cost.mana ([generic 1]))))
theorem okFadeAway : Action.check (DRS.ofList []) fadeAway = [] := by decide

def zimoneAndDina : Ability :=
  activated
      (compound [tapSymbol, (Semantics.Cost.perform (sacrifice (a (and [isCreature, isOther]))))])
    (sequentially
      [ draw 1,
        may (putOntoBattlefieldTapped (a (and [isLand, in_ yourHand]))),
        doIf (compareAmt (countOf (and [isLand, youControl])) .atLeast 8)
          (repeat_ (moreTimes 1)) ])
theorem okZimoneAndDina : Ability.check (DRS.ofList []) zimoneAndDina = [] := by decide

/-- Memoricide -/
def memoricideSearch : Action :=
  sequentially
    [ choose (a (qualityFrom .cardName (nameOfCard (non isLand)))),
      searchZonesOf (target isPlayer) (counted anyNumber (named chosenName)),
      exile them,
      instruct (that player) shuffle ]
theorem okMemoricideSearch : Action.check (DRS.ofList []) memoricideSearch = [] := by decide
/-- Lost Hours -/
def lostHoursPlacement : Action :=
  sequentially
    [ instruct (target isPlayer) revealHand,
      choose (a (and [non isLand, in_ (handOf they)])),
      instruct (that player) (put it (fromTop 3 library)) ]
theorem okLostHoursPlacement : Action.check (DRS.ofList []) lostHoursPlacement = [] := by decide
/-- Aether Gust -/
def aetherGustPlacement : Action :=
  sequentially
    [ choose (target (and [or [isSpell, isPermanent], or [hasColor .red, hasColor .green]])),
      instruct (ownerOf it) (put it (or [onTop, onBottom])) ]
theorem okAetherGustPlacement : Action.check (DRS.ofList []) aetherGustPlacement = [] := by decide
/-- Fastbond -/
def fastbondLands : Ability := static (can (Semantics.Subject.actor (you)) (play (counted (additional anyNumber) isLand)))
theorem okFastbondLands : Ability.check (DRS.ofList []) fastbondLands = [] := by decide
/-- Panglacial Wurm -/
def panglacialWurmCast : Ability :=
  static
      (can (Semantics.Subject.actor (you)) (cast this_ (from_ := yourLibrary)) (timing := whileSearchingYourLibrary))
theorem okPanglacialWurmCast : Ability.check (DRS.ofList []) panglacialWurmCast = [] := by decide

/-- Tranquil Frillback -/
def tranquilFrillbackOffer : Action := may (pay (Semantics.Cost.mana ([pip .green])) (.upTo 3))
theorem okTranquilFrillbackOffer : Action.check (DRS.ofList []) tranquilFrillbackOffer = [] := by decide

/-- Duneblast -/
def duneblast : Action :=
  (match (Semantics.Cards.«Duneblast») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ part) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okDuneblast : Action.check (DRS.ofList []) duneblast = [] := by decide
/-- Boreas Charger -/
def boreasChargerChoice : Action := choose (a opponentWithMoreLands)
theorem okBoreasChargerChoice : Action.check (DRS.ofList []) boreasChargerChoice = [] := by decide
/-- Boreas Charger's spell text -/
def boreasChargerSpell : Action :=
  sequentially
    [ choose (a opponentWithMoreLands),
      search yourLibrary (counted (exactlyOf theDifference) (Semantics.Predicate.hasSubtype (plains))),
      reveal (those card),
      putOntoBattlefieldTapped (someOf (exactly 1) (those card)),
      putInto theRest hand ]
theorem okBoreasChargerSpell : Action.check (DRS.ofList []) boreasChargerSpell = [] := by decide

/-- Slithermuse -/
def slithermuseTrigger : Ability :=
  when (does (Semantics.Subject.actor (thisCreature)) leave)
    (sequentially
      [ choose anOpponent,
        doIf (compareAmt (countOf (cardIn (handOf (that player)))) .greater
                (countOf (cardIn yourHand)))
          (draw theDifference) ])
theorem okSlithermuseTrigger : Ability.check (DRS.ofList []) slithermuseTrigger = [] := by decide

/-- World Queller -/
def worldQuellerChoice : Action :=
  may (choose (a (quality .cardType)))
    (instruct eachPlayer (sacrifice (aTheirChoice (and [isPermanent, ofChosen .cardType]))))
theorem okWorldQuellerChoice : Action.check (DRS.ofList []) worldQuellerChoice = [] := by decide

def discardUpToTwoThenDrawThatMany : Action :=
  sequentially [discard (counted (upTo 2) (cardIn yourHand)), draw groupSize]
theorem okDiscardUpToTwoThenDrawThatMany :
    Action.check (DRS.ofList []) discardUpToTwoThenDrawThatMany = [] := by decide
/-- Truce and Temporary Truce -/
def drawUpToTwoThenGainPerShortfall : Action :=
  instruct eachPlayer
    (sequentially
      [ may (drawCount (upToOf 2)),
        instruct (that player) (gainLife (times 2 shortOfCeiling)) ])
theorem okDrawUpToTwoThenGainPerShortfall :
    Action.check (DRS.ofList []) drawUpToTwoThenGainPerShortfall = [] := by decide

/-- Nautiloid Ship -/
def nautiloidShipTrigger : Ability :=
  whenever (does (Semantics.Subject.actor (thisVehicle)) (dealDamage (recipient := a isPlayer) (kind := some .combat)))
    (may (putOntoBattlefieldUnderYourControl (a (and [isCreature, in_ (exiledWith thisVehicle)]))))
theorem okNautiloidShipTrigger : Ability.check (DRS.ofList []) nautiloidShipTrigger = [] := by decide

/-- Summon: Esper Valigarmanda's II/III/IV body, in part -/
def summonEsperValigarmandaCast : StaticSpec :=
  conjunction [ can (Semantics.Subject.actor (you)) (cast (a (and [isInstantOrSorcery, in_ (exiledWith thisSaga)]))),
      can (Semantics.Subject.actor (you)) (pay anyCost .once)
          (asThough :=
             spendManaAs none .anyType
                (toCast (and [isInstantOrSorcery, in_ (exiledWith thisSaga)]))) ]
theorem okSummonEsperValigarmandaCast :
    StaticSpec.check (DRS.ofList []) summonEsperValigarmandaCast = [] := by decide
/-- Rogue Class's level-3 body -/
def rogueClassLevelThree : StaticSpec :=
  conjunction [ can (Semantics.Subject.actor (you)) (play (allOf (in_ (exiledWith thisClass)))),
      can (Semantics.Subject.actor (you)) (pay anyCost .once)
          (asThough :=
             spendManaAs none .anyColor (toCast (in_ (exiledWith thisClass)))) ]
theorem okRogueClassLevelThree : StaticSpec.check (DRS.ofList []) rogueClassLevelThree = [] := by decide

/-- Muldrotha, the Gravetide -/
def muldrothaLandWindow : StaticSpec :=
  can (Semantics.Subject.actor (you)) (play (a isLand) (from_ := yourGraveyard)) (timing := onEachOfYourTurns)
theorem okMuldrothaLandWindow : StaticSpec.check (DRS.ofList []) muldrothaLandWindow = [] := by decide
/-- Nahiri's Lithoforming -/
def nahiriExtraLands : StaticSpec := can (Semantics.Subject.actor (you))
    (play (counted (additional (exactlyOf (letter .x))) isLand))
theorem okNahiriExtraLands : StaticSpec.check (DRS.ofList [letterB .x]) nahiriExtraLands = [] := by decide

/-- Apex of Power -/
def apexOfPowerCast : Action :=
  sequentially
    [ exile (topSlice 7),
      establishThroughout (can (Semantics.Subject.actor (you)) (cast (fromAmong anyNumber isSpell them))) thisTurn ]
theorem okApexOfPowerCast : Action.check (DRS.ofList []) apexOfPowerCast = [] := by decide
/-- Blight Herder -/
def blightHerderCast : Action :=
  may
    (putInto (counted (exactly 2)
            (and [isCard, opponentsOwn, in_ exileZone]))
      graveyard)
theorem okBlightHerderCast : Action.check (DRS.ofList []) blightHerderCast = [] := by decide

/-- Akiri, Fearless Voyager -/
def akiriUnattachOffer : Ability :=
  activated (Semantics.Cost.mana ([pip .white]))
    (may
      (unattach (a (and [ (Semantics.Predicate.hasSubtype (equipment)),
                            attachedTo (a (and [isCreature, youControl])) ])))
      (tap (that creature)))
theorem okAkiriUnattachOffer : Ability.check (DRS.ofList []) akiriUnattachOffer = [] := by decide
/-- Summoning Materia -/
def summoningMateriaTopCast : Ability :=
  static (onlyWhile ((can (Semantics.Subject.actor (you)) (cast (bare (and [isSpell, isCreature])) (from_ := onTop))))
    (is thisEquipment (attachedTo (a isCreature))))
theorem okSummoningMateriaTopCast : Ability.check (DRS.ofList []) summoningMateriaTopCast = [] := by decide
/-- Vizier of the Menagerie -/
def vizierOfTheMenagerieSpend : StaticSpec :=
  can (Semantics.Subject.actor (you)) (pay anyCost .once)
      (asThough := spendManaAs none .anyType (toCast (and [isCreature, isSpell])))
theorem okVizierOfTheMenagerieSpend : StaticSpec.check (DRS.ofList []) vizierOfTheMenagerieSpend = [] := by
  decide

/-- Emissary of Grudges -/
def emissaryOfGrudgesReveal : Ability :=
  activatedOnlyOnce (Semantics.Cost.perform ((expose .reveal (choiceOf .player))))
    (doOnlyIf (chooseNewTargets (target (or [isSpell, abilityHead .anyOnStack])))
      (and
        [ is it (controlledBy (the isChosenPlayer)),
          is it
            (targeting (youOr (a (and [isPermanent, youControl]))) .someTarget) ]))
    oncePerGame
theorem okEmissaryOfGrudgesReveal :
    Ability.check (DRS.ofList [ChoiceSort.binding .player]) emissaryOfGrudgesReveal = [] := by decide

def collectiveUnconscious : Action := draw (forEach 1 (and [isCreature, youControl]))
theorem okCollectiveUnconscious : Action.check (DRS.ofList []) collectiveUnconscious = [] := by decide
def killiansConfidence : Action :=
  sequentially
    [get (target isCreature) (up 1) (up 1) untilEndOfTurn, draw 1]
theorem okKilliansConfidence : Action.check (DRS.ofList []) killiansConfidence = [] := by decide

/-- Not Forgotten -/
def notForgottenPlacement : Action :=
  putInto (target (cardIn graveyard)) (or [onTop, onBottom])
theorem okNotForgottenPlacement : Action.check (DRS.ofList []) notForgottenPlacement = [] := by decide

def boreasChargerDifference : Amount := theDifference
theorem okBoreasChargerDifference :
    Amount.check ((stepAction 512 .postEvent .obj (DRS.ofList []) boreasChargerChoice).1) boreasChargerDifference = [] := by decide

/-- Stick Together: a chosen party is four up-to-one choices, not the group the game computes
[CR#700.8d]. -/
def stickTogether : Action :=
  sequentially
    [ instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, (Semantics.Predicate.hasSubtype (cleric)), controlledBy they]))),
      instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, (Semantics.Predicate.hasSubtype (rogue)), controlledBy they]))),
      instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, (Semantics.Predicate.hasSubtype (warrior)), controlledBy they]))),
      instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, (Semantics.Predicate.hasSubtype (wizard)), controlledBy they]))),
      instruct eachPlayer (sacrifice theRest) ]
theorem okStickTogether : Action.check (DRS.ofList []) stickTogether = [] := by decide

/-- In *Loxodon Peacekeeper*, "them" after the counted existential reads the players it
introduced (`.count`), not the player with the lowest life total. -/
theorem loxodonPeacekeeperThemReadsTheTiedPlayers :
    let lowest := the (and [isPlayer, superlative .min (.playerStat .lifeTotal) isPlayer])
    let before := (stepAction 512 .postEvent .obj (DRS.ofList [])
      (gainControl thisCreature (controller := lowest))).1
    let tied := thereAre (counted (orMore 2)
      (and [isPlayer, tiedFor .min (.playerStat .lifeTotal) isPlayer]))
    let after := (stepCondition 512 (nextClause before) tied).1
    (resolveCandidates (.word .player) .many (nextClause after)).map (·.det) = [.count] := by
  decide

private def juxtaposeTie : Action :=
  (match (Semantics.Cards.«Juxtapose») with
    | .singleFaced face => match face.characteristics.text with
      | .spell _ (.sequentially (_ :: _ :: action :: _)) :: _ => some action
      | _ => none
    | _ => none).get (by decide)

/-- In *Juxtapose*, "them" reads the tied permanents the counted existential introduces
(`.count`), even after an ordinary plural mention of artifacts ("artifacts you control get
+1/+1"): an existential that is no replacement's condition ranks as the nearest mention (owner
ruling 2026-10-22, batch 4). -/
theorem okJuxtaposeThemReadsTheTiedPermanents :
    let tied := thereAre (counted (orMore 2)
      (and [isPermanent, controlledBy (a isPlayer),
            tiedFor .max (.stat .manaValue) (and [isPermanent, controlledBy (that player)])]))
    let artifacts := (stepAction 512 .postEvent .obj (DRS.ofList [])
      (get (bare (and [isArtifact, youControl])) (up 1) (up 1) untilEndOfTurn)).1
    let after := (stepCondition 512 (nextClause artifacts) tied).1
    Action.check (DRS.ofList [])
        (sequentially [get (bare (and [isArtifact, youControl])) (up 1) (up 1) untilEndOfTurn,
          juxtaposeTie]) = [] ∧
      (artifacts.findById? 1).map (·.det) = some .all ∧
      (resolveCandidates .bare .many (nextClause after)).map (·.det) = [.count] := by
  decide

end Semantics.Cards

end
