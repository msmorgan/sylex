import Semantics.Cards
import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsAnaphora

/-!
# Semantics.Cards.Trigger

Port of `idris/src/Experimental/Cards/Trigger.idr`: the printed cards of the Trigger family
(headers, intervening-ifs, joined and alternative headers, delayed and replacement shapes) and
the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def gracefulReprieve : Action :=
  delayWithin (does (target isCreature) die) thisTurn
    (returnToBattlefield (that card) [(under (ownerOf it))])
theorem okGracefulReprieve : Action.check (DRS.ofList []) gracefulReprieve = [] := by decide

def cloudkinSeer : Ability := when (does thisCreature enter) (draw 1)
theorem okCloudkinSeer : Ability.check (DRS.ofList []) cloudkinSeer = [] := by decide
def promiseOfTomorrow : Ability := whenever (does (a (and [isCreature, youControl])) die) (exile it)
theorem okPromiseOfTomorrow : Ability.check (DRS.ofList []) promiseOfTomorrow = [] := by decide
def libraryLarcenist : Ability := whenever (does thisCreature attack) (draw 1)
theorem okLibraryLarcenist : Ability.check (DRS.ofList []) libraryLarcenist = [] := by decide
def jhessianThief : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
    (draw 1)
theorem okJhessianThief : Ability.check (DRS.ofList []) jhessianThief = [] := by decide
def scholarOfStars : Ability :=
  when (does thisCreature enter)
    (if_ := exists_ (and [isArtifact, youControl])) (draw 1)
theorem okScholarOfStars : Ability.check (DRS.ofList []) scholarOfStars = [] := by decide

def miserysShadow : Ability :=
  static (replace
            (would (a (and [isCreature, anOpponentControls])) die) (instead (exile it)))
theorem okMiserysShadow : Ability.check (DRS.ofList []) miserysShadow = [] := by decide

def beastWhisperer : Ability :=
  whenever (does you (cast (a (and
    [isCreature, isSpell])))) (draw 1)
theorem okBeastWhisperer : Ability.check (DRS.ofList []) beastWhisperer = [] := by decide
def mesmericOrb : Ability :=
  whenever (does (a isPermanent) (becomeStatus untapped))
    (instruct (controllerOf (that permanent)) (mill 1))
theorem okMesmericOrb : Ability.check (DRS.ofList []) mesmericOrb = [] := by decide
def secretPlans : Ability :=
  whenever (does any (turnFaceUp (a (and [isPermanent, youControl]))))
    (draw 1)
theorem okSecretPlans : Ability.check (DRS.ofList []) secretPlans = [] := by decide

/-- Teferi's Imp -/
def teferisImpPhasesOut : Ability :=
  whenever (does thisCreature phaseOut) (discard (a (cardIn yourHand)))
theorem okTeferisImpPhasesOut : Ability.check (DRS.ofList []) teferisImpPhasesOut = [] := by decide
/-- Teferi's Imp -/
def teferisImpPhasesIn : Ability :=
  whenever (does thisCreature phaseIn) (draw 1)
theorem okTeferisImpPhasesIn : Ability.check (DRS.ofList []) teferisImpPhasesIn = [] := by decide

/-- Oubliette -/
def oubliette : Ability :=
  when (does thisEnchantment enter)
    (phaseOutUntil (target isCreature) (does thisEnchantment leave))
theorem okOubliette : Ability.check (DRS.ofList []) oubliette = [] := by decide

def shimmeringEfreet : Ability :=
  whenever (does thisCreature phaseIn) (instruct (target isCreature) phaseOut)
theorem okShimmeringEfreet : Ability.check (DRS.ofList []) shimmeringEfreet = [] := by decide
def hollowhengeSpirit : Ability :=
  when (does thisCreature enter)
    (removeFromCombat (target (and [isCreature, or [isAttacking, isBlocking]])))
theorem okHollowhengeSpirit : Ability.check (DRS.ofList []) hollowhengeSpirit = [] := by decide
def netcasterSpider : Ability :=
  whenever (does thisCreature (block (a (and [isCreature, hasKeyword flying]))))
    (get thisCreature (up 2) (up 0) untilEndOfTurn)
theorem okNetcasterSpider : Ability.check (DRS.ofList []) netcasterSpider = [] := by decide
def viashinoWeaponsmith : Ability :=
  whenever (does (by_ (a isCreature)) (block thisCreature))
    (get thisCreature (up 2) (up 2) untilEndOfTurn)
theorem okViashinoWeaponsmith : Ability.check (DRS.ofList []) viashinoWeaponsmith = [] := by decide
def somberwaldAlpha : Ability :=
  whenever (does any (becomeBlocked (a (and [isCreature, youControl]))))
    (get it (up 1) (up 1) untilEndOfTurn)
theorem okSomberwaldAlpha : Ability.check (DRS.ofList []) somberwaldAlpha = [] := by decide
def vertigoSpawn : Ability :=
  whenever (does thisCreature (block (a isCreature)))
    (sequentially
      [ tap (that creature),
        skipUntap (that creature) 1 ])
theorem okVertigoSpawn : Ability.check (DRS.ofList []) vertigoSpawn = [] := by decide
def orneryDilophosaur : Ability :=
  whenever (does thisCreature attack)
    (if_ := exists_ (and [ isCreature, youControl,
                     hasPower .atLeast 4 ]))
    (get thisCreature (up 2) (up 2) untilEndOfTurn)
theorem okOrneryDilophosaur : Ability.check (DRS.ofList []) orneryDilophosaur = [] := by decide
def incisorGlider : Ability :=
  whenever (does thisCreature attack)
    (if_ := compareAmt (countersOn poison anOpponent) .atLeast 3)
    (get (allOf (and [isCreature, youControl])) (up 1) (up 1) untilEndOfTurn)
theorem okIncisorGlider : Ability.check (DRS.ofList []) incisorGlider = [] := by decide
def stormFleetSpy : Ability :=
  when (does thisCreature enter) (if_ := some (did you attack soFarThisTurn))
    (draw 1)
theorem okStormFleetSpy : Ability.check (DRS.ofList []) stormFleetSpy = [] := by decide
def loanShark : Ability :=
  when (does thisCreature enter)
    (if_ := compareAmt (eventCount (cast any) you soFarThisTurn) .atLeast 2) (draw
      1)
theorem okLoanShark : Ability.check (DRS.ofList []) loanShark = [] := by decide

def forceOfDespair : Action :=
  destroy (allOf
             (and
                [isCreature, did which enter soFarThisTurn]))
theorem okForceOfDespair : Action.check (DRS.ofList []) forceOfDespair = [] := by decide
def cradleToGrave : Action :=
  destroy (target (and [isCreature, not
    (hasColor .black), did which enter soFarThisTurn]))
theorem okCradleToGrave : Action.check (DRS.ofList []) cradleToGrave = [] := by decide

def aragornKingOfGondor : Ability :=
  when (does thisCreature enter) (gainDesignation you monarch)
theorem okAragornKingOfGondor : Ability.check (DRS.ofList []) aragornKingOfGondor = [] := by decide
def firmamentSage : Ability := whenever (gameBecomes night) (draw 1)
theorem okFirmamentSage : Ability.check (DRS.ofList []) firmamentSage = [] := by decide
def deeprootWarrior : Ability :=
  whenever (does any (becomeBlocked thisCreature))
    (get it (up 1) (up 1) untilEndOfTurn)
theorem okDeeprootWarrior : Ability.check (DRS.ofList []) deeprootWarrior = [] := by decide
def borderlandMarauder : Ability :=
  whenever (does thisCreature attack) (get it (up 2) (up 0) untilEndOfTurn)
theorem okBorderlandMarauder : Ability.check (DRS.ofList []) borderlandMarauder = [] := by decide
def lichsMasteryLoss : Ability :=
  when (does thisEnchantment leave) loseGame
theorem okLichsMasteryLoss : Ability.check (DRS.ofList []) lichsMasteryLoss = [] := by decide
def phageTheUntouchable : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
      (instruct (that player) loseGame)
theorem okPhageTheUntouchable : Ability.check (DRS.ofList []) phageTheUntouchable = [] := by decide
def elderscaleWurm : Ability :=
  when (does thisCreature enter) (if_ := compareAmt (lifeTotalOf you) .less 7)
    (setLife 7)
theorem okElderscaleWurm : Ability.check (DRS.ofList []) elderscaleWurm = [] := by decide
/-- Krang, Master Mind -/
def krang : Ability :=
  when (does thisCreature enter)
    (if_ := compareAmt (countOf (cardIn yourHand)) .less 4) (draw theDifference)
theorem okKrang : Ability.check (DRS.ofList []) krang = [] := by decide

/-- Once Upon a Time -/
def onceUponATimeFirstCast : Predicate :=
  did you (cast which) thisObject (ordinal := some (.nth 1, some .game))
theorem okOnceUponATimeFirstCast : Predicate.checkAt .object (DRS.ofList []) onceUponATimeFirstCast = [] := by
  decide

def archivistOfGondor : Ability :=
  when (does yourCommander (dealDamage (to := a isPlayer) (kind := some .combat)))
      (if_ := thereIsNo monarch)
    (gainDesignation you monarch)
theorem okArchivistOfGondor : Ability.check (DRS.ofList []) archivistOfGondor = [] := by decide

/-- Giggling Skitterspike -/
def gigglingSkitterspikeTriggers : List GameEvent :=
  [does thisCreature attack, does thisCreature block, does (a isSpell) (target thisCreature)]
theorem okGigglingSkitterspikeTriggers :
    GameEvent.checkAll (DRS.ofList []) gigglingSkitterspikeTriggers = [] := by decide

/-- Stalwart Successor -/
def stalwartSuccessorHeader : GameEvent := does any (putCounters anyAmount anyKind (a (and [isCreature, youControl])))
theorem okStalwartSuccessorHeader : GameEvent.check (DRS.ofList []) stalwartSuccessorHeader = [] := by decide

/-- Hollowmurk Siege -/
def hollowmurkSiegeSultai : Ability :=
  whenever (does any (putCounters 1 anyKind (a (and [isCreature, youControl]))))
    (draw 1) (limit := oncePerTurn)
theorem okHollowmurkSiegeSultai : Ability.check (DRS.ofList []) hollowmurkSiegeSultai = [] := by decide

/-- Sakashima's Student -/
def sakashimasStudentCopy : Ability :=
  static (enters thisCreature
    [asCopyOf true (a isCreature) [isAlso [] [ninja]]])
theorem okSakashimasStudentCopy : Ability.check (DRS.ofList []) sakashimasStudentCopy = [] := by decide
/-- Chameleon, Master of Disguise -/
def chameleonCopy : Ability :=
  static (enters thisCreature
    [asCopyOf true (a (and [isCreature, youControl]))
      [isNamed "Chameleon, Master of Disguise"]])
theorem okChameleonCopy : Ability.check (DRS.ofList []) chameleonCopy = [] := by decide

def nefariousLichGain : Ability :=
  static (replace (would you (gainLife anyAmount)) (instead (draw thatMuch)))
theorem okNefariousLichGain : Ability.check (DRS.ofList []) nefariousLichGain = [] := by decide

/-- Souls of the Faultless -/
def soulsOfTheFaultlessDrain : Ability :=
  whenever (does any (dealDamage (to := thisCreature) (kind := some .combat)))
    (instruct (combatPlayer .attacking) (loseLife thatMuch))
theorem okSoulsOfTheFaultlessDrain : Ability.check (DRS.ofList []) soulsOfTheFaultlessDrain = [] := by decide

/-- Steppe Lynx -/
def steppeLynx : Ability :=
  landfall
    (whenever (does (a (and [isLand, youControl])) enter)
      (get thisCreature (up 2) (up 2) untilEndOfTurn))
theorem okSteppeLynx : Ability.check (DRS.ofList []) steppeLynx = [] := by decide
/-- Owlbear -/
def owlbear : Ability :=
  flavorWord "Keen Senses" (when (does thisCreature enter) (draw 1))
theorem okOwlbear : Ability.check (DRS.ofList []) owlbear = [] := by decide
/-- Netherese Puzzle-Ward -/
def netheresePuzzleWardIllumination : Ability :=
  whenever (does you (rollDice (watch := highestNatural))) (draw 1)
theorem okNetheresePuzzleWardIllumination :
    Ability.check (DRS.ofList []) netheresePuzzleWardIllumination = [] := by decide
/-- Farideh, Devil's Chosen -/
def faridehResultRead : Ability :=
  whenever (does you (rollDice anyAmount .anyDie))
    (sequentially
      [ gain thisCreature flying untilEndOfTurn,
        gain thisCreature menace untilEndOfTurn,
        doIf (anyResultIs .atLeast 10) (draw 1) ])
theorem okFaridehResultRead : Ability.check (DRS.ofList []) faridehResultRead = [] := by decide
/-- Jaws of Defeat -/
def jawsOfDefeat : Ability :=
  whenever (does (a (and [isCreature, youControl])) enter)
    (instruct (target isOpponent) (loseLife
      (arith .differenceBetween (powerOf (that creature))
        (toughnessOf (that creature)))))
theorem okJawsOfDefeat : Ability.check (DRS.ofList []) jawsOfDefeat = [] := by decide
/-- Defiling Daemogoth -/
def defilingDaemogothDrain : Action :=
  sequentially
    [instruct eachOpponent (loseLife (letter .x)),
      define .x (eventSum (gainLife anyAmount) you soFarThisTurn)]
theorem okDefilingDaemogothDrain : Action.check (DRS.ofList []) defilingDaemogothDrain = [] := by decide
def skullsporeNexusTrigger : Ability :=
  whenever (does (counted (orMore 1) (and [isNontoken, and [isCreature, youControl]])) die)
    (create 1 (creatureTokenOf (totalOf (.stat .power) (those card))
        (totalOf (.stat .power) (those card))
        [.green] [fungus, dinosaur]))
theorem okSkullsporeNexusTrigger : Ability.check (DRS.ofList []) skullsporeNexusTrigger = [] := by decide
/-- Wavebreak Hippocamp -/
def wavebreakHippocamp : Ability :=
  whenever (nthOccurrence (.nth 1) (does you (cast (a isSpell))))
    (during := duringPart .turn eachOpponent) (draw 1)
theorem okWavebreakHippocamp : Ability.check (DRS.ofList []) wavebreakHippocamp = [] := by decide
/-- Midnight Clock -/
def midnightClockHeader : GameEvent :=
  does any (putCounters (theNth (.nth 12)) hourCounter thisArtifact)
theorem okMidnightClockHeader : GameEvent.check (DRS.ofList []) midnightClockHeader = [] := by decide
/-- Political Triumph -/
def politicalTriumphHeader : GameEvent :=
  does any (putCounters (theNth (.nth 4)) planCounter thisEnchantment)
theorem okPoliticalTriumphHeader : GameEvent.check (DRS.ofList []) politicalTriumphHeader = [] := by decide
/-- Thought Lash -/
def thoughtLashTrigger : Ability :=
  when (does (a isPlayer)
          (pay anyCost .once .unpaid (some thisEnchantment) (some cumulativeUpkeep)))
    (instruct (that player) (exile (each (cardIn (libraryOf they)))))
theorem okThoughtLashTrigger : Ability.check (DRS.ofList []) thoughtLashTrigger = [] := by decide
/-- Heart of Bogardan -/
def heartOfBogardanHeader : GameEvent :=
  (match heartOfBogardan.card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.triggered part _ _ _ _ _ _ _) _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okHeartOfBogardanHeader : GameEvent.check (DRS.ofList []) heartOfBogardanHeader = [] := by decide
/-- Balduvian Fallen -/
def balduvianFallenHeader : GameEvent := does any (pay anyCost .once .paid (some thisCreature) (some cumulativeUpkeep))
theorem okBalduvianFallenHeader : GameEvent.check (DRS.ofList []) balduvianFallenHeader = [] := by decide
/-- Stormscape Battlemage -/
def stormscapeBattlemageFirstKicker : Ability :=
  when (does thisCreature enter)
    (if_ := costWasPaid (.byNthKeyword (.nth 1) kicker) none thisCreature)
    (gainLife 3)
theorem okStormscapeBattlemageFirstKicker :
    Ability.check (DRS.ofList []) stormscapeBattlemageFirstKicker = [] := by decide
/-- All-Seeing Arbiter -/
def allSeeingArbiterHeader : GameEvent :=
  does you (discard (a
    (cardIn hand)))
theorem okAllSeeingArbiterHeader : GameEvent.check (DRS.ofList []) allSeeingArbiterHeader = [] := by decide

/-- Tahngarth, First Mate -/
def tahngarthHeader : GameEvent := does anOpponent (attackWith (counted (orMore 1) isCreature))
theorem okTahngarthHeader : GameEvent.check (DRS.ofList []) tahngarthHeader = [] := by decide
/-- Myth Unbound -/
def mythUnboundTrigger : Ability :=
  whenever (does any (putInto yourCommander commandZone (from_ := some any))) (draw 1)
theorem okMythUnboundTrigger : Ability.check (DRS.ofList []) mythUnboundTrigger = [] := by decide

/-- Oscorp Industries -/
def oscorpIndustriesReturn : Ability :=
  when (does thisLand (enter (from_ := graveyard))) (loseLife 2)
theorem okOscorpIndustriesReturn : Ability.check (DRS.ofList []) oscorpIndustriesReturn = [] := by decide
def theLostAndTheDamnedEntryArm : GameEvent :=
  does (a (and [isLand, youControl])) (enter (from_ := some (not yourHand)))
theorem okTheLostAndTheDamnedEntryArm :
    GameEvent.check (DRS.ofList []) theLostAndTheDamnedEntryArm = [] := by decide
/-- Forsaken Wastes -/
def forsakenWastesTargeted : Ability :=
  whenever (does (a isSpell) (target thisEnchantment))
    (instruct (controllerOf (that .spell)) (loseLife 5))
theorem okForsakenWastesTargeted : Ability.check (DRS.ofList []) forsakenWastesTargeted = [] := by decide
/- *Fblthp, the Lost*'s self-replacement "draw a card. If …, draw two cards instead" checks: a
regression guard on the replacement's starting state [CR#614.15]. -/

/-- Frost Walker's "it" resolves to the source: its graveyard successor (`2`) records the
battlefield source's id (`1`) in `movedFrom`, leaving the targeting spell or ability (`0`) alone. -/
theorem frostWalkerSacrificesItself :
    let out := (stepAction 64 .postEvent .obj
      (nextClause (stepGameEvent 64 (DRS.ofList [])
        (does (a (or [isSpell, abilityHead .anyOnStack])) (target thisCreature))).1)
      (sacrifice it)).1
    frostWalker.card.check = [] ∧ out.resolvedIds = [2] ∧
      (out.findById? 2).map (fun ref => (ref.det, ref.zone, ref.movedFrom)) =
        some (.self, some .graveyard, [1]) ∧
      (out.findById? 0).bind (·.zone) = some .stack := by
  decide

/-- Segmented Wurm's "it" is the source (`1`), not the targeting spell or ability (`0`). -/
theorem segmentedWurmPutsTheCounterOnItself :
    segmentedWurm.card.check = [] ∧
    (stepAction 64 .postEvent .obj
      (nextClause (stepGameEvent 64 (DRS.ofList [])
        (does (a (or [isSpell, abilityHead .anyOnStack])) (target thisCreature))).1)
      (putCounters 1 m1m1Counter it)).1.resolvedIds = [1] := by
  decide

/-- Angelic Cub's trigger reads "it" as the source (`1`), not the targeting spell or ability
(`0`); the trigger fragment keeps its first occurrence each turn. -/
theorem angelicCubPutsTheCounterOnItself :
    angelicCub.card.check = [] ∧
    (stepAction 64 .postEvent .obj
      (nextClause (stepGameEvent 64 (DRS.ofList [])
        (nthOccurrence (.nth 1)
          (does (a (or [isSpell, abilityHead .anyOnStack])) (target thisCreature)) (some .turn))).1)
      (putCounters 1 p1p1Counter it)).1.resolvedIds = [1] := by
  decide

/-- Debris Beetle -/
def debrisBeetleTrigger : Ability :=
  (match debrisBeetle.card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons part _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okDebrisBeetleTrigger : Ability.check (DRS.ofList []) debrisBeetleTrigger = [] := by decide
/-- Roads Go Ever, Ever On's chapters II and III -/
def roadsGoEverEverOnChapters : Ability :=
  when (chapterMark [2, 3]) (putInto (a (cardIn (exiledWith thisSaga))) hand)
theorem okRoadsGoEverEverOnChapters : Ability.check (DRS.ofList []) roadsGoEverEverOnChapters = [] := by decide
/-- Wurmwall Sweeper -/
def wurmwallSweeperTrigger : Ability := when (does thisSpacecraft enter) (surveil 2)
theorem okWurmwallSweeperTrigger : Ability.check (DRS.ofList []) wurmwallSweeperTrigger = [] := by decide
/-- Case of the Crimson Pulse -/
def caseOfTheCrimsonPulseTrigger : Ability :=
  when (does thisCase enter)
    (sequentially [discard (a (cardIn yourHand)), draw 2])
theorem okCaseOfTheCrimsonPulseTrigger : Ability.check (DRS.ofList []) caseOfTheCrimsonPulseTrigger = [] := by
  decide

/-- Agency Outfitter's search -/
def agencyOutfitterSearch : Action :=
  sequentially
    [ may
        (sequentially
          [ search (andOr [yourGraveyard, yourHand, yourLibrary])
              (andOr [a (named "Magnifying Glass"), a (named "Thinking Cap")]),
            putOntoBattlefield them ]),
      doIf (did you (search yourLibrary) thisWay) shuffle ]
theorem okAgencyOutfitterSearch : Action.check (DRS.ofList []) agencyOutfitterSearch = [] := by decide
/-- Agency Outfitter's "them" reads the and/or group of the cards it searched for. -/
theorem agencyOutfitterThemIsTheFoundGroup :
    let found := nextClause (stepAction 512 .postEvent .obj (DRS.ofList [])
      (search (andOr [yourGraveyard, yourHand, yourLibrary])
        (andOr [a (named "Magnifying Glass"), a (named "Thinking Cap")]))).1
    (resolveCandidates .bare .many found).map (fun ref => (ref.plur, ref.groupKey == ref.id)) =
      [(.many, true)] := by decide

/-- Trouble in Pairs' -/
def troubleInPairsArms : Ability :=
  whenever (does anOpponent (attackWith (counted (orMore 2) isCreature) you))
    (or_ :=
      [ nthOccurrence (.nth 2) (does (actor anOpponent) (draw anyAmount)) (some .turn),
        nthOccurrence (.nth 2) (does anOpponent (cast (a isSpell))) (some .turn) ])
    (draw 1)
theorem okTroubleInPairsArms : Ability.check (DRS.ofList []) troubleInPairsArms = [] := by decide

private def castRank? : Predicate → Option (Ordinal × Option Period)
  | .did clause => clause.ordinal
  | _ => none

private def eventOrdinal? : GameEvent → Option (Ordinal × Option Period)
  | .nthOccurrence ordinal per _ => (ordinal, per)
  | _ => none

private def triggerAlternatives : Ability → List GameEvent
  | .triggered _ alternatives .. => alternatives
  | _ => []

/-- "The first spell you've cast this game" and "your second spell each turn" carry one ordinal
shape, an `Ordinal` counted within an optional `Period`. -/
theorem onceUponATimeAndTroubleInPairsShareTheOrdinalShape :
    castRank? onceUponATimeFirstCast = some (.nth 1, some .game) ∧
      (triggerAlternatives troubleInPairsArms).map eventOrdinal? =
        [some (.nth 2, some .turn), some (.nth 2, some .turn)] := by decide

/-- Hostile Investigator -/
def hostileInvestigatorHeader : GameEvent :=
  does (counted (orMore 1) isPlayer) (discard (counted (orMore 1) isCard))
theorem okHostileInvestigatorHeader : GameEvent.check (DRS.ofList []) hostileInvestigatorHeader = [] := by decide

/-- Don't Blink's replacement, without its written agent -/
def dontBlinkReplacement : Action :=
  establishThroughout
    (replace
      (wouldAny
        [ would (counted (orMore 1) isCreature) (enter (from_ := exileZone)),
          would (counted (orMore 1) (and [isCreature,
              did any (cast which (from_ := exileZone)) thisObject])) enter ])
      (instead (shuffleInto them)))
    untilEndOfTurn
theorem okDontBlinkReplacement : Action.check (DRS.ofList []) dontBlinkReplacement = [] := by decide

/-- Autarch Mammoth -/
def autarchMammothLine : Ability :=
  (match autarchMammoth.card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons part _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okAutarchMammothLine : Ability.check (DRS.ofList []) autarchMammothLine = [] := by decide
/-- Altar of the Brood -/
def altarOfTheBrood : Ability :=
  whenever (does (a (and [isPermanent, youControl, isOther])) enter)
    (instruct eachOpponent (mill 1))
theorem okAltarOfTheBrood : Ability.check (DRS.ofList []) altarOfTheBrood = [] := by decide
/-- Foul Emissary -/
def foulEmissaryLine : Ability :=
  when (does you (sacrifice thisCreature))
    (while_ := whileDoing (does you (cast (a
      (and [isSpell, hasKeyword (.the emerge)])))))
    (create 1 (creatureToken 3 2 [] [eldrazi, horror]))
theorem okFoulEmissaryLine : Ability.check (DRS.ofList []) foulEmissaryLine = [] := by decide

/-- Cloud, Ex-SOLDIER -/
def cloudExSoldierAttach : Ability :=
  when (does thisCreature enter)
    (attachToIt
      (targets (upTo 1) (and [equipment, youControl])))
theorem okCloudExSoldierAttach : Ability.check (DRS.ofList []) cloudExSoldierAttach = [] := by decide
/-- Akiri, Fearless Voyager -/
def akiriEquippedAttackers : Ability :=
  whenever
    (does you
       (attackWith
          (counted (orMore 1) (and [and [isCreature, youControl], isAttached (some .equipped)]))
        (a isPlayer)))
    (draw 1)
theorem okAkiriEquippedAttackers : Ability.check (DRS.ofList []) akiriEquippedAttackers = [] := by decide
/-- Vexing Bauble -/
def vexingBaubleTrigger : Ability :=
  whenever (does (a isPlayer) (cast (a isSpell)))
    (if_ := noManaSpentToCast it)
    (counter (that .spell))
theorem okVexingBaubleTrigger : Ability.check (DRS.ofList []) vexingBaubleTrigger = [] := by decide

/-- Jeskai Ascendancy's first trigger -/
def jeskaiAscendancyPump : Ability :=
  whenever (does you (cast (a (and
    [isSpell, non isCreature]))))
    (sequentially
      [ get (bare (and [isCreature, youControl])) (up 1) (up 1) untilEndOfTurn,
        untap (those creature) ])
theorem okJeskaiAscendancyPump : Ability.check (DRS.ofList []) jeskaiAscendancyPump = [] := by decide
def herosDemise : Action := destroy (target (and [isCreature, hasSupertype .legendary]))
theorem okHerosDemise : Action.check (DRS.ofList []) herosDemise = [] := by decide

theorem bioplasmTwoCandidates : countOnes .object bioplasmAfterExile = 2 := by decide

/-- Curie, Emergent Intelligence's first ability: "Whenever Curie deals combat damage to a
player, draw cards equal to its base power" [CR#208.4b]. -/
def curieCombatDraw : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
    (draw (basePowerOf thisCreature))
theorem okCurieCombatDraw : Ability.check (DRS.ofList []) curieCombatDraw = [] := by decide

end Semantics.Cards
