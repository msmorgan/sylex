import Semantics.Cards.Data.Fixtures
import Semantics.Check
import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsHelpers

-- Canon cards are `noncomputable` data (`Semantics.Cards.Data`); so is what reads them.
noncomputable section


/-!
# Semantics.Cards.Damage

Port of `idris/src/Experimental/Cards/Damage.idr`: the printed cards of the Damage family and
the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def barrageOfBoulders : Action :=
  instruct this_ (dealDamage 1 (each (and [isCreature, youDontControl])))
theorem okBarrageOfBoulders : Action.check (DRS.ofList []) barrageOfBoulders = [] := by decide
def rabidBite : Action :=
  dealDamageOwnPower (target (and [isCreature, youControl]))
    (target (and [isCreature, youDontControl]))
theorem okRabidBite : Action.check (DRS.ofList []) rabidBite = [] := by decide
def preyUpon : Action := instruct (target (and [isCreature, youControl])) (fight (target (and [isCreature, youDontControl])))
theorem okPreyUpon : Action.check (DRS.ofList []) preyUpon = [] := by decide
def arcTrail : Action :=
  instruct this_ (sequentially
    [ dealDamage 2 (target isAnyTarget), dealDamage 1 (target isAnyOtherTarget) ])
theorem okArcTrail : Action.check (DRS.ofList []) arcTrail = [] := by decide
def deadshot : Action :=
  sequentially
    [ tap (target isCreature),
      dealDamageOwnPower it (target (and [isCreature, isOther])) ]
theorem okDeadshot : Action.check (DRS.ofList []) deadshot = [] := by decide
def immersturmSkullcairn : Ability :=
  activatedOnlyDuring
    (compound [[generic 1, pip .black, pip .red, pip .red], tapSymbol,
      (sacrifice thisLand)])
    (sequentially
      [ instruct this_ (dealDamage 3 (target isPlayer)),
        instruct (that .player) (discard (a (cardIn hand))) ])
    asSorcery
theorem okImmersturmSkullcairn : Ability.check (DRS.ofList []) immersturmSkullcairn = [] := by decide
def pyriteSpellbomb : Ability :=
  activated (compound [[pip .red], sacrifice thisArtifact])
    (instruct this_ (dealDamage 2 (target isAnyTarget)))
theorem okPyriteSpellbomb : Ability.check (DRS.ofList []) pyriteSpellbomb = [] := by decide

def suddenDemise : Action :=
  sequentially
    [ choose (a (quality .color)),
      instruct this_ (dealDamage (letter .x) (each (and [isCreature, ofChosen .color]))) ]
theorem okSuddenDemise : Action.check (DRS.ofList []) suddenDemise = [] := by decide
def caseOfTheGatewayExpress : Action :=
  sequentially
    [ choose (target (and [isCreature, youDontControl])),
      instruct (each (and [isCreature, youControl]))
          (dealDamage 1 (that creature)) ]
theorem okCaseOfTheGatewayExpress : Action.check (DRS.ofList []) caseOfTheGatewayExpress = [] := by decide
def arrowsOfJustice : Action :=
  instruct this_ (dealDamage 4 (target (and [isCreature, or [isAttacking, isBlocking]])))
theorem okArrowsOfJustice : Action.check (DRS.ofList []) arrowsOfJustice = [] := by decide
def flamesOfTheRazeBoar : Action :=
  sequentially
    [ instruct this_ (dealDamage 4 (target (and [isCreature, anOpponentControls]))),
      doOnlyIf
        (instruct this_
          (dealDamage 2 (each (and [isCreature, isOther, controlledBy (that player)]))))
        (exists_ (and [isCreature, youControl, hasPower .atLeast 4])) ]
theorem okFlamesOfTheRazeBoar : Action.check (DRS.ofList []) flamesOfTheRazeBoar = [] := by decide
def yawgmothDemon : Action :=
  mayOrElse (sacrifice (a isArtifact)) (sequentially
                                          [tap thisCreature, instruct this_
                                             (dealDamage 2 you)])
theorem okYawgmothDemon : Action.check (DRS.ofList []) yawgmothDemon = [] := by decide
def arcBlade : Action :=
  sequentially
    [ instruct this_ (dealDamage 2 (target isAnyTarget)),
      exileWithCounters this_ 3 timeCounter ]
theorem okArcBlade : Action.check (DRS.ofList []) arcBlade = [] := by decide
def abrade : Action :=
  chooseOne (instruct this_ (dealDamage 3 (target isCreature)))
    (destroy (target isArtifact))
theorem okAbrade : Action.check (DRS.ofList []) abrade = [] := by decide
def nibelheimAflame : Action :=
  sequentially
    [ choose (target (and [isCreature, youControl])),
      dealDamageOwnPower it (each (and [isCreature, isOther])) ]
theorem okNibelheimAflame : Action.check (DRS.ofList []) nibelheimAflame = [] := by decide
def brashTaunter : Action := instruct thisCreature
    (fight (target (and [isCreature, isOther])))
theorem okBrashTaunter : Action.check (DRS.ofList []) brashTaunter = [] := by decide
def ulvenwaldTracker : Action :=
  instruct (target (and [isCreature, youControl])) (fight (target (and [isCreature, isOther])))
theorem okUlvenwaldTracker : Action.check (DRS.ofList []) ulvenwaldTracker = [] := by decide

def botBashingTime : Action :=
  sequentially
    [ instruct this_ (dealDamage 6 (target isCreature)),
      replaceEvent (would (that creature) die)
          (instead (exile it)) thisTurn ]
theorem okBotBashingTime : Action.check (DRS.ofList []) botBashingTime = [] := by decide
def wordsOfWar : Action :=
  replaceNextEvent (would you (draw 1))
      (instead (instruct this_ (dealDamage 2 (target isAnyTarget)))) thisTurn
theorem okWordsOfWar : Action.check (DRS.ofList []) wordsOfWar = [] := by decide

/-- Fog, Holy Day, Darkness, Root Snare -/
def fog : Action := preventAll any thisTurn (some .combat)
theorem okFog : Action.check (DRS.ofList []) fog = [] := by decide
/-- Indestructible Aura, Shielded Passage -/
def indestructibleAura : Action :=
  preventAll (target isCreature) thisTurn
theorem okIndestructibleAura : Action.check (DRS.ofList []) indestructibleAura = [] := by decide
def shieldmatesBlessing : Action :=
  preventNext (target isAnyTarget) 3 thisTurn
theorem okShieldmatesBlessing : Action.check (DRS.ofList []) shieldmatesBlessing = [] := by decide
def moonlitWake : Ability := (match (cardNamed "Moonlit Wake").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons part _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okMoonlitWake : Ability.check (DRS.ofList []) moonlitWake = [] := by decide
def eliteJavelineer : Ability :=
  whenever (does thisCreature block)
      (instruct this_
         (dealDamage 1 (target (and [isCreature, isAttacking]))))
theorem okEliteJavelineer : Ability.check (DRS.ofList []) eliteJavelineer = [] := by decide
/-- Glacial Chasm -/
def glacialChasmShield : Ability :=
  (match (cardNamed "Glacial Chasm").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons _ (List.cons part _)))) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okGlacialChasmShield : Ability.check (DRS.ofList []) glacialChasmShield = [] := by decide
def corneredCrook : Ability :=
  when (does thisCreature enter)
    (offerWhen (sacrifice (a isArtifact)) (instruct this_ (dealDamage 3 (target
        isAnyTarget))))
theorem okCorneredCrook : Ability.check (DRS.ofList []) corneredCrook = [] := by decide

def chandrasRevolution : Action :=
  sequentially
    [ instruct this_ (dealDamage 4 (target isCreature)), tap (target isLand),
      skipUntap (that land) 1 ]
theorem okChandrasRevolution : Action.check (DRS.ofList []) chandrasRevolution = [] := by decide
def arbalestElite : Ability :=
  activated (compound [[generic 2, pip .white], tapSymbol])
    (sequentially
      [ instruct thisCreature (dealDamage 3 (target (and [isCreature, or [isAttacking, isBlocking]]))),
        skipUntap thisCreature 1 ])
theorem okArbalestElite : Ability.check (DRS.ofList []) arbalestElite = [] := by decide
def sizzlingBarrage : Action :=
  instruct this_ (dealDamage 4 (target
    (and [isCreature, did which block soFarThisTurn])))
theorem okSizzlingBarrage : Action.check (DRS.ofList []) sizzlingBarrage = [] := by decide
def goadedAttackTrigger : Ability :=
  whenever (does (a (and [isCreature, hasDesignation goaded])) attack)
    (instruct it (dealDamage 1 (controllerOf it)))
theorem okGoadedAttackTrigger : Ability.check (DRS.ofList []) goadedAttackTrigger = [] := by decide

/-- Purging Scythe: "If two or more creatures are tied for least toughness, you choose one of
them." The tie-break is written as the choice of one creature with the least toughness, which "that creature" reads. -/
def purgingScythe : Ability :=
  at_ (beginningOfYour .upkeep)
    (sequentially
      [ choose (a (and [isCreature, superlative .min (.stat .toughness) isCreature])),
        instruct thisArtifact (dealDamage 2 (that creature)) ])
theorem okPurgingScythe : Ability.check (DRS.ofList []) purgingScythe = [] := by decide

/-- Chandra Nalaar -/
def chandraNalaarsX : Ability :=
  activated (loyaltySymbol (down (letter .x))) (instruct this_ (dealDamage (letter .x) (target isCreature)))
theorem okChandraNalaarsX : Ability.check (DRS.ofList []) chandraNalaarsX = [] := by decide

def onakkeJavelineerBolt : Ability :=
  activated tapSymbol
    (instruct thisCreature (dealDamage 2 (target (or [isBattle, isPlayer]))))
theorem okOnakkeJavelineerBolt : Ability.check (DRS.ofList []) onakkeJavelineerBolt = [] := by decide

/-- Firesong and Sunspeaker -/
def firesongJoinEcho : Action :=
  sequentially
    [ instruct this_ (dealDamage 3 (target (or [isCreature, isPlayer]))),
      instruct this_ (dealDamage 1 thatJoin) ]
theorem okFiresongJoinEcho : Action.check (DRS.ofList []) firesongJoinEcho = [] := by decide

def glarecasterShield : Ability :=
  activated [generic 5, pip .white]
    (establishThroughout
      (replace (would any (dealDamage anyAmount (andOr [thisCreature, you])))
        (instead (redirectDamage allDamage (target isAnyTarget))) (use := .nextTimeOnly))
      thisTurn)
theorem okGlarecasterShield : Ability.check (DRS.ofList []) glarecasterShield = [] := by decide

def saheeliRaiPlusOne : Action :=
  sequentially [scry 1, instruct this_ (dealDamage 1 eachOpponent)]
theorem okSaheeliRaiPlusOne : Action.check (DRS.ofList []) saheeliRaiPlusOne = [] := by decide
def sarkhansUnsealingLine : Ability :=
  whenever
    (does you (cast (a (and [isCreature, isSpell,
                or [ hasPower .eq 4, hasPower .eq 5,
                      hasPower .eq 6 ]]))))
    (instruct thisEnchantment (dealDamage 4 (target isAnyTarget)))
theorem okSarkhansUnsealingLine : Ability.check (DRS.ofList []) sarkhansUnsealingLine = [] := by decide
/-- Savage Swipe, both sentences -/
def savageSwipeLine : Action :=
  sequentially
    [ doOnlyIf (get (target (and [isCreature, youControl])) (up 2) (up 2)
        untilEndOfTurn)
        (compareAmt (powerOf it) .eq 2),
      instruct it (fight (target (and [isCreature, youDontControl]))) ]
theorem okSavageSwipeLine : Action.check (DRS.ofList []) savageSwipeLine = [] := by decide

def galvanicBlastLine : Action :=
  replace (instruct this_ (dealDamage 2 (target isAnyTarget)))
    (doIf (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 3)
      (instead (instruct this_ (dealDamage 4 thatJoin))))
theorem okGalvanicBlastLine : Action.check (DRS.ofList []) galvanicBlastLine = [] := by decide
/-- Furious Reprisal -/
def furiousReprisal : Action :=
  instruct this_ (dealDamage 2 (eachOf (targets (exactly 2) isAnyTarget)))
theorem okFuriousReprisal : Action.check (DRS.ofList []) furiousReprisal = [] := by decide
/-- Bullseye, Death Dealer -/
def bullseyeModalCost : Ability :=
  activated
    (compound [[generic 3], tapSymbol,
      (chooseOne (sacrifice (a isArtifact))
                 (discard (a (and [non isLand, in_ yourHand]))))])
    (instruct this_ (dealDamage 2 (target isAnyTarget)))
theorem okBullseyeModalCost : Ability.check (DRS.ofList []) bullseyeModalCost = [] := by decide

/-- Quakebringer: "This ability triggers only if …" restricts when the ability triggers, as
Solved's does [CR#702.169c]; it is no intervening "if", which only immediately follows a trigger
condition and is rechecked on resolution [CR#603.4]. So it is the `qualified` guard, and the
ability resolves even if Quakebringer leaves in response. The guard also names the zones it
functions from [CR#113.6b], the battlefield or the graveyard, which `qualified`'s one-zone `from_`
cannot. -/
def quakebringerDamage : Ability :=
  qualified
    (at_ (beginningOfYour .upkeep) (instruct this_ (dealDamage 2 eachOpponent)))
    (guard := some (or [ is this_ (in_ battlefield),
           and [ is this_ (cardIn yourGraveyard),
                  exists_ (and [isCreature, giant,
                                 youControl]) ] ]))
theorem okQuakebringerDamage : Ability.check (DRS.ofList []) quakebringerDamage = [] := by decide
/-- Sand Strangler -/
def sandStranglerDamage : Ability :=
  when (does thisCreature enter)
    (if_ := some (or [ exists_ (and [isLand, desert, youControl]),
           exists_ (and [isLand, desert, in_ yourGraveyard]) ]))
    (may (instruct this_ (dealDamage 3 (target isCreature))))
theorem okSandStranglerDamage : Ability.check (DRS.ofList []) sandStranglerDamage = [] := by decide

/-- Lavalanche -/
def lavalanche : Action :=
  instruct this_ (simultaneously
    [ dealDamage (letter .x) targetPlayerOrPlaneswalker,
      dealDamage (letter .x) eachCreatureThatSplitControls ])
theorem okLavalanche : Action.check (DRS.ofList []) lavalanche = [] := by decide
/-- Flame Wave -/
def flameWave : Action :=
  instruct this_ (simultaneously
    [ dealDamage 4 targetPlayerOrPlaneswalker,
      dealDamage 4 eachCreatureThatSplitControls ])
theorem okFlameWave : Action.check (DRS.ofList []) flameWave = [] := by decide
/-- Chandra Nalaar's ultimate -/
def chandraNalaarUltimate : Action :=
  simultaneously
    [ instruct this_ (dealDamage 10 targetPlayerOrPlaneswalker),
      instruct this_ (dealDamage 10 eachCreatureThatSplitControls) ]
theorem okChandraNalaarUltimate : Action.check (DRS.ofList []) chandraNalaarUltimate = [] := by decide
/-- Chandra, Pyrogenius's ultimate -/
def chandraPyrogeniusUltimate : Action :=
  simultaneously
    [ instruct this_ (dealDamage 6 targetPlayerOrPlaneswalker),
      instruct this_ (dealDamage 6 eachCreatureThatSplitControls) ]
theorem okChandraPyrogeniusUltimate : Action.check (DRS.ofList []) chandraPyrogeniusUltimate = [] := by decide
/-- Bonfire of the Damned -/
def bonfireOfTheDamned : Action :=
  instruct this_ (simultaneously
    [ dealDamage (letter .x) targetPlayerOrPlaneswalker,
      dealDamage (letter .x) eachCreatureThatSplitControls ])
theorem okBonfireOfTheDamned : Action.check (DRS.ofList []) bonfireOfTheDamned = [] := by decide
/-- Chandra's Fury -/
def chandrasFury : Action :=
  instruct this_ (simultaneously
    [ dealDamage 4 targetPlayerOrPlaneswalker,
      dealDamage 1 eachCreatureThatSplitControls ])
theorem okChandrasFury : Action.check (DRS.ofList []) chandrasFury = [] := by decide
/-- Angrath, Minotaur Pirate's plus -/
def angrathMinotaurPirateBolt : Action :=
  instruct this_ (simultaneously
    [ dealDamage 1 targetOpponentOrPlaneswalker,
      dealDamage 1 eachCreatureThatSplitControls ])
theorem okAngrathMinotaurPirateBolt : Action.check (DRS.ofList []) angrathMinotaurPirateBolt = [] := by decide
def whichOfYouBurnsBrightestBody : Action :=
  simultaneously
    [ dealDamage (letter .x) targetOpponentOrPlaneswalker,
      dealDamage (letter .x) eachCreatureThatSplitControls ]
theorem okWhichOfYouBurnsBrightestBody :
    Action.check (DRS.ofList []) whichOfYouBurnsBrightestBody = [] := by decide
/-- Chandra, Pyromaster's plus -/
def chandraPyromasterBolt : Action :=
  simultaneously
    [ instruct this_ (dealDamage 1 targetPlayerOrPlaneswalker),
      instruct this_ (dealDamage 1
                         (targets (upTo 1)
                            (and
                               [isCreature, controlledBy splitOverPlaneswalker]))) ]
theorem okChandraPyromasterBolt : Action.check (DRS.ofList []) chandraPyromasterBolt = [] := by decide
/-- Ravager of the Fells -/
def ravagerOfTheFellsBolt : Action :=
  simultaneously
    [ instruct thisCreature (dealDamage 2 targetOpponentOrPlaneswalker),
      instruct thisCreature (dealDamage 2
                               (targets (upTo 1)
                                  (and
                                     [isCreature, controlledBy
                                        splitOverPlaneswalker]))) ]
theorem okRavagerOfTheFellsBolt : Action.check (DRS.ofList []) ravagerOfTheFellsBolt = [] := by decide
/-- Soul of Shandalar's battlefield activation -/
def soulOfShandalarBolt : Action :=
  simultaneously
    [ instruct thisCreature (dealDamage 3 targetPlayerOrPlaneswalker),
      instruct thisCreature (dealDamage 3
                               (targets (upTo 1)
                                  (and
                                     [isCreature, controlledBy
                                        splitOverPlaneswalker]))) ]
theorem okSoulOfShandalarBolt : Action.check (DRS.ofList []) soulOfShandalarBolt = [] := by decide
/-- Soul of Shandalar's graveyard activation -/
def soulOfShandalarGraveyardBolt : Action :=
  simultaneously
    [ instruct this_ (dealDamage 3 targetPlayerOrPlaneswalker),
      instruct this_ (dealDamage 3
                         (targets (upTo 1)
                            (and
                               [isCreature, controlledBy splitOverPlaneswalker]))) ]
theorem okSoulOfShandalarGraveyardBolt :
    Action.check (DRS.ofList []) soulOfShandalarGraveyardBolt = [] := by decide
/-- Blightning -/
def blightning : Action :=
  sequentially
    [ instruct this_ (dealDamage 3 targetPlayerOrPlaneswalker),
      discard (counted (exactly 2) (cardIn (handOf splitOverPlaneswalker))) ]
theorem okBlightning : Action.check (DRS.ofList []) blightning = [] := by decide
/-- Rakdos's Return -/
def rakdossReturn : Action :=
  sequentially
    [ instruct this_ (dealDamage (letter .x) targetOpponentOrPlaneswalker),
      discard (counted (exactlyOf (letter .x)) (cardIn (handOf splitOverPlaneswalker))) ]
theorem okRakdossReturn : Action.check (DRS.ofList []) rakdossReturn = [] := by decide
/-- Nicol Bolas, Planeswalker's ultimate -/
def nicolBolasUltimate : Action :=
  sequentially
    [ instruct this_ (dealDamage 7 targetPlayerOrPlaneswalker),
      discard (counted (exactly 7) (cardIn (handOf splitOverPlaneswalker))),
      instruct splitOverPlaneswalker (sacrifice (counted (exactly 7) isPermanent)) ]
theorem okNicolBolasUltimate : Action.check (DRS.ofList []) nicolBolasUltimate = [] := by decide
/-- Pulse of the Forge -/
def pulseOfTheForge : Action :=
  sequentially
    [ instruct this_ (dealDamage 4 targetPlayerOrPlaneswalker),
      doIf (compareAmt (lifeTotalOf splitOverPlaneswalker) .greater (lifeTotalOf you))
        (returnToHand this_) ]
theorem okPulseOfTheForge : Action.check (DRS.ofList []) pulseOfTheForge = [] := by decide
/-- Goblin Lyre's losing arm -/
def goblinLyreLoseFlip : Action :=
  sequentially
    [ instruct thisArtifact (dealDamage (countOf (and [isCreature, youControl])) targetOpponentOrPlaneswalker),
      instruct thisArtifact (dealDamage
                               (countOf
                                  (and
                                     [isCreature, controlledBy
                                        splitOverPlaneswalker])) you) ]
theorem okGoblinLyreLoseFlip : Action.check (DRS.ofList []) goblinLyreLoseFlip = [] := by decide

/-- Searing Blaze, both sentences -/
def searingBlaze : Ability :=
  landfall
    (spell (replace
        (instruct this_ (simultaneously
          [ dealDamage 1 targetPlayerOrPlaneswalker,
            dealDamage 1 (target
                                   (and
                                      [isCreature, controlledBy
                                         splitOverPlaneswalker])) ]))
        (doIf (did (a (and [isLand, youControl])) enter soFarThisTurn)
          (instead
            (instruct this_ (simultaneously
              [ dealDamage 3 thatJoin,
                dealDamage 3 (that creature) ]))))))
theorem okSearingBlaze : Ability.check (DRS.ofList []) searingBlaze = [] := by decide

/-- Explosive Prodigy -/
def explosiveProdigyTrigger : Ability :=
  vivid
    (when (does thisCreature enter)
      (sequentially
        [ instruct it
            (dealDamage (letter .x) (target (and [isCreature, anOpponentControls]))),
          define .x (distinctCount .color (allOf (and [isPermanent, youControl]))) ]))
theorem okExplosiveProdigyTrigger : Ability.check (DRS.ofList []) explosiveProdigyTrigger = [] := by decide
/-- Niv-Mizzet, Guildpact -/
def nivMizzetGuildpactTrigger : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
    (sequentially
      [ instruct thisCreature (dealDamage (letter .x) (target isAnyTarget)),
        instruct (target isPlayer) (draw (letter .x)),
        gainLife (letter .x),
        define .x (distinctCount .colorPair
          (allOf (and [isPermanent, youControl, colorCount .eq 2]))) ])
theorem okNivMizzetGuildpactTrigger : Ability.check (DRS.ofList []) nivMizzetGuildpactTrigger = [] := by decide

/-- Breeches, Brazen Plunderer's slice -/
def eachOfThoseOpponentsTopCard : Action :=
  sequentially
    [ instruct this_ (dealDamage 1 eachOpponent),
      exile (slice .top 1 (libraryOf (eachOf (those player)))) ]
theorem okEachOfThoseOpponentsTopCard : Action.check (DRS.ofList []) eachOfThoseOpponentsTopCard = [] := by
  decide

/-- Crackling Doom -/
def cracklingDoom : Action :=
  sequentially
    [ instruct this_ (dealDamage 2 eachOpponent),
      instruct eachOpponent (sacrifice
        (a (and [isCreature,
                  superlative .max (.stat .power)
                    (and [isCreature, controlledBy (that player)])]))) ]
theorem okCracklingDoom : Action.check (DRS.ofList []) cracklingDoom = [] := by decide
def theFallen : Ability :=
  at_ (beginningOfYour .upkeep)
    (instruct thisCreature (dealDamage 1 (each (and [or [isOpponent, isPlaneswalker],
                   did thisCreature (dealDamage anyAmount which) thisGame]))))
theorem okTheFallen : Ability.check (DRS.ofList []) theFallen = [] := by decide
/-- Cavalcade of Calamity -/
def cavalcadeOfCalamity : Ability :=
  whenever
    (does (a (and [isCreature, youControl, hasPower .atMost 1])) attack)
    (instruct thisEnchantment (dealDamage 1 (the (and [or [isPlayer, isPlaneswalker],
                  inCombat .attackedBy (that creature)]))))
theorem okCavalcadeOfCalamity : Ability.check (DRS.ofList []) cavalcadeOfCalamity = [] := by decide

/-- Chandra, Awakened Inferno's emblem -/
def chandraAwakenedInfernoEmblem : Action :=
  getEmblem
    [ at_ (beginningOfYour .upkeep)
        (instruct thisEmblem (dealDamage 1 you)) ]
theorem okChandraAwakenedInfernoEmblem :
    Action.check (DRS.ofList []) chandraAwakenedInfernoEmblem = [] := by decide

def diabolicEdict : Action := instruct (target isPlayer) (sacrifice (aTheirChoice isCreature))
theorem okDiabolicEdict : Action.check (DRS.ofList []) diabolicEdict = [] := by decide
def innocentBlood : Action := instruct eachPlayer (sacrifice (aTheirChoice isCreature))
theorem okInnocentBlood : Action.check (DRS.ofList []) innocentBlood = [] := by decide
def cryOfContrition : Action := instruct (target isPlayer) (discard (a (cardIn hand)))
theorem okCryOfContrition : Action.check (DRS.ofList []) cryOfContrition = [] := by decide
def cyclingCost : Action := discard this_
theorem okCyclingCost : Action.check (DRS.ofList []) cyclingCost = [] := by decide
def raiseTheAlarm : Action :=
  create 2 (creatureToken 1 1 [.white] [soldier])
theorem okRaiseTheAlarm : Action.check (DRS.ofList []) raiseTheAlarm = [] := by decide
def actOfTreason : Action := gainControl (target isCreature) untilEndOfTurn (controller :=
    you)
theorem okActOfTreason : Action.check (DRS.ofList []) actOfTreason = [] := by decide
def wordsOfWorship : Action :=
  replaceNextEvent (would you (draw 1)) (instead (gainLife 5)) thisTurn
theorem okWordsOfWorship : Action.check (DRS.ofList []) wordsOfWorship = [] := by decide
def theLastRoninII : Action :=
  triggerReflexively (mill 4)
    (returnToHand (target (and [isCreature, in_ yourGraveyard])) yourGraveyard)
theorem okTheLastRoninII : Action.check (DRS.ofList []) theLastRoninII = [] := by decide

end Semantics.Cards

end
