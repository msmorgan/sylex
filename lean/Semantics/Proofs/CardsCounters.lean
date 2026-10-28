import Semantics.Cards
import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Counters

Port of `idris/src/Experimental/Cards/Counters.idr`: the printed cards of the Counters family
and the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Crovax the Cursed -/
def crovaxTheCursed : Action :=
  may (sacrifice (a isCreature))
    (putCounters 1 p1p1Counter thisCreature)
    (removeCounters (exactly 1) p1p1Counter thisCreature)
theorem okCrovaxTheCursed : Action.check (DRS.ofList []) crovaxTheCursed = [] := by decide
def additiveEvolution : Action :=
  sequentially
    [ create 1 (creatureToken 0 0 [.green, .blue] [fractal]),
      putCounters 3 p1p1Counter it ]
theorem okAdditiveEvolution : Action.check (DRS.ofList []) additiveEvolution = [] := by decide
def battlegrowth : Action := putCounters 1 p1p1Counter (target isCreature)
theorem okBattlegrowth : Action.check (DRS.ofList []) battlegrowth = [] := by decide
def chainbreaker : Action :=
  removeCounters (exactly 1) m1m1Counter (target isCreature)
theorem okChainbreaker : Action.check (DRS.ofList []) chainbreaker = [] := by decide
def kaitoBaneOfNightmares : Action :=
  sequentially
    [tap (target isCreature), putCounters 2 stunCounter it]
theorem okKaitoBaneOfNightmares : Action.check (DRS.ofList []) kaitoBaneOfNightmares = [] := by decide
def jhoiraOfTheGhitu : Ability :=
  activated
    (compound [[generic 2], exile (a (and [non isLand, in_ yourHand]))])
    (putCounters 4 timeCounter (the (and [was (exile which), isCard])))
theorem okJhoiraOfTheGhitu : Ability.check (DRS.ofList []) jhoiraOfTheGhitu = [] := by decide
def alaundoTheSeer : Action :=
  removeCounters (exactly 1) timeCounter (each
                                                                            (and
                                                                               [youOwn, in_
                                                                                  exileZone]))
theorem okAlaundoTheSeer : Action.check (DRS.ofList []) alaundoTheSeer = [] := by decide

def ashnodsTransmogrant : Ability :=
  activated (compound [tapSymbol, sacrifice thisArtifact])
    (sequentially
      [ putCounters 1 p1p1Counter (target (and [isCreature, non isArtifact])),
        become (that creature) { characteristics := { types := [.artifact] } } ])
theorem okAshnodsTransmogrant : Ability.check (DRS.ofList []) ashnodsTransmogrant = [] := by decide
def azulaAlwaysLies : Action :=
  chooseModes (oneThrough 2)
    [ get (target isCreature) (down 1) (down 1) untilEndOfTurn,
      putCounters 1 p1p1Counter (target isCreature) ]
theorem okAzulaAlwaysLies : Action.check (DRS.ofList []) azulaAlwaysLies = [] := by decide
def bellowingAegisaur : Action :=
  putCounters 1 p1p1Counter
      (each (and [isCreature, youControl, isOther]))
theorem okBellowingAegisaur : Action.check (DRS.ofList []) bellowingAegisaur = [] := by decide
def carnifexDemon : Action :=
  putCounters 1 m1m1Counter (each (and [isCreature, isOther]))
theorem okCarnifexDemon : Action.check (DRS.ofList []) carnifexDemon = [] := by decide
def ajaniAdversaryOfTyrants : Action :=
  putCounters 1 p1p1Counter (eachOf (targets (upTo 2) isCreature))
theorem okAjaniAdversaryOfTyrants : Action.check (DRS.ofList []) ajaniAdversaryOfTyrants = [] := by decide
def naturesPanoply : Action :=
  sequentially
    [ choose (targets anyNumber isCreature),
      putCounters 1 p1p1Counter (eachOf them) ]
theorem okNaturesPanoply : Action.check (DRS.ofList []) naturesPanoply = [] := by decide
def armamentCorps : Action :=
  distributeCounters 2 p1p1Counter
    (targets (oneThrough 2) (and [isCreature, youControl]))
theorem okArmamentCorps : Action.check (DRS.ofList []) armamentCorps = [] := by decide
def savagebornHydra : Ability :=
  activatedOnlyDuring [generic 1, hybridPip .red .green]
    (putCounters 1 p1p1Counter thisCreature) asSorcery
theorem okSavagebornHydra : Ability.check (DRS.ofList []) savagebornHydra = [] := by decide
def woeleecher : Ability :=
  (match (cardNamed "Woeleecher").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons part _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okWoeleecher : Ability.check (DRS.ofList []) woeleecher = [] := by decide
def anointerOfValor : Ability :=
  whenever (does (a isCreature) attack)
    (offerWhen (pay [generic 3] .once)
      (putCounters 1 p1p1Counter (that creature)))
theorem okAnointerOfValor : Ability.check (DRS.ofList []) anointerOfValor = [] := by decide
def avianOddity : Action :=
  putCounters 1 flyingCounter (target (and [isCreature, youControl]))
theorem okAvianOddity : Action.check (DRS.ofList []) avianOddity = [] := by decide
/-- Song of Eärendil -/
def songOfEarendil : Action :=
  putCounters 1 flyingCounter
    (each (and [isCreature, youControl, not (hasKeyword flying)]))
theorem okSongOfEarendil : Action.check (DRS.ofList []) songOfEarendil = [] := by decide
def gideonsAvenger : Ability :=
  whenever (does (a (and [isCreature, anOpponentControls])) (becomeStatus tapped))
    (putCounters 1 p1p1Counter thisCreature)
theorem okGideonsAvenger : Ability.check (DRS.ofList []) gideonsAvenger = [] := by decide
def prologueToPhyresis : Action :=
  putCounters 1 poison eachOpponent
theorem okPrologueToPhyresis : Action.check (DRS.ofList []) prologueToPhyresis = [] := by decide
def screechingScorchbeast : Ability :=
  whenever (does thisCreature attack) (putCounters 2 radCounter eachPlayer)
theorem okScreechingScorchbeast : Ability.check (DRS.ofList []) screechingScorchbeast = [] := by decide
def merenOfClanNelToth : Ability :=
  whenever (does (a (and [isCreature, youControl, isOther])) die)
    (putCounters 1 experience you)
theorem okMerenOfClanNelToth : Ability.check (DRS.ofList []) merenOfClanNelToth = [] := by decide
def kratosStoicFather : Ability :=
  at_ (beginningOfYour .endStep)
    (putCounters (countersOn experience you) p1p1Counter
      (target isCreature))
theorem okKratosStoicFather : Ability.check (DRS.ofList []) kratosStoicFather = [] := by decide
def vashtaNerada : Ability :=
  at_ (beginningOfPossessed .endStep eachPlayer)
    (if_ := some (did (a isCreature) die soFarThisTurn))
    (putCounters 1 p1p1Counter thisCreature)
theorem okVashtaNerada : Ability.check (DRS.ofList []) vashtaNerada = [] := by decide
def furiousSpinesplitter : Ability :=
  at_ (beginningOfYour .endStep)
    (putCounters (forEach 1 (and
      [isOpponent, was (dealDamage anyAmount which) soFarThisTurn]))
      p1p1Counter thisCreature)
theorem okFuriousSpinesplitter : Ability.check (DRS.ofList []) furiousSpinesplitter = [] := by decide
def paladinOfAtonement : Ability :=
  at_ (beginningOfPossessed .upkeep eachPlayer)
    (if_ := some (did you (loseLife anyAmount) lastTurn))
    (putCounters 1 p1p1Counter thisCreature)
theorem okPaladinOfAtonement : Ability.check (DRS.ofList []) paladinOfAtonement = [] := by decide
def throneWarden : Ability :=
  at_ (beginningOfYour .endStep)
    (if_ := is you (hasDesignation monarch))
    (putCounters 1 p1p1Counter thisCreature)
theorem okThroneWarden : Ability.check (DRS.ofList []) throneWarden = [] := by decide
def bloodTyrant : Ability :=
  whenever (does (a isPlayer) loseGame)
      (putCounters 5 p1p1Counter thisCreature)
theorem okBloodTyrant : Ability.check (DRS.ofList []) bloodTyrant = [] := by decide

def toweringTitan : Ability :=
  static (conjunction [ entersWithCounters thisCreature (letter .x) p1p1Counter,
      letterDefinition .x
        (aggregate .sum (.stat .toughness)
           (and [isCreature, youControl, isOther])) ])
theorem okToweringTitan : Ability.check (DRS.ofList []) toweringTitan = [] := by decide

/-- Corpsejack Menace -/
def corpsejackMenace : Ability :=
  static (replace (would any (putCounters anyAmount p1p1Counter (a (and [isCreature, youControl])))) (instead (putCounters (times 2 thatMuch) p1p1Counter it)))
theorem okCorpsejackMenace : Ability.check (DRS.ofList []) corpsejackMenace = [] := by decide

/-- Doubling Season's first line: "If an effect would create one or more tokens under your
control, it creates twice that many of those tokens instead." -/
def doublingSeasonTokens : Ability :=
  (match (cardNamed "Doubling Season").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons part _) => some part
      | _ => none
    | _ => none).get (by decide)

/-- Doubling Season's second line: "If an effect would put one or more counters on a permanent
you control, it puts twice that many of those counters on that permanent instead." -/
def doublingSeasonCounters : Ability :=
  (match (cardNamed "Doubling Season").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons part _)) => some part
      | _ => none
    | _ => none).get (by decide)

/-- Whether an ability is a replacement whose would-clause an effect causes. -/
private def watchesAnEffect : Ability → Bool
  | .static (.replace (.causedBy .anEffect _) ..) => true
  | _ => false

/-- Both of Doubling Season's lines watch only what an effect does, "if an effect would …"
[CR#614.16], so neither doubles a Saga's turn-based lore counter [CR#703.1,714.3c]. Corpsejack
Menace's "if one or more +1/+1 counters would be put" has no such cause. -/
theorem doublingSeasonWatchesAnEffect :
    watchesAnEffect doublingSeasonTokens = true ∧ watchesAnEffect doublingSeasonCounters = true ∧
      watchesAnEffect corpsejackMenace = false := by decide

/-- What *Doc Samson, Super Psychiatrist* watches you do: "put one or more counters on a permanent
you control". -/
def docSamsonWatched : Action := putCounters anyAmount anyKind (a (and [isPermanent, youControl]))

/-- Doc Samson, Super Psychiatrist -/
def docSamsonDistributive : Ability :=
  static (replace (would you docSamsonWatched)
    (instead (putCounters (plus thatMuch 1) thoseCounters (that permanent))))
theorem okDocSamsonDistributive : Ability.check (DRS.ofList []) docSamsonDistributive = [] := by decide

/-- Adapt is an instruction about "this permanent" that you, its controller, follow
[CR#701.46a,608.2c], so the counters it puts are ones you put: Doc Samson's "if you would put"
watches them. -/
theorem docSamsonWatchesAdaptsCounters :
    let watched := docSamsonWatched.describedEvents (some .player)
    let adapted := (stepAction 64 .postEvent .obj (DRS.ofList []) (adapt 2)).1.causedEvents
    watched ≠ [] ∧ watched.all adapted.contains = true := by
  decide

/-- *Biomancer's Familiar*: "{T}: The next time target creature adapts this turn, it adapts as
though it had no +1/+1 counters on it." Its "target creature adapts" describes the adapt the
creature undergoes, done by a player, as a resolving "target creature adapts 2" causes it: the
creature is what the adapt acts on and carries its stamp, not the Source [CR#701.46a,608.2c].
The card itself is unspelled until `lean-v3-single-use-as-though` gives an as-though statement
its single use. -/
theorem biomancersFamiliarWatchesTheCreatureThatAdapts :
    let described := (adapt anyAmount).describedEvents (some .object)
    let adapted := (stepGameEvent 64 (DRS.ofList []) (.does (.actor (target isCreature)) (adapt 2))).1
    let resolved := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (instruct (target isCreature) (adapt 2))).1.causedEvents
    described.map (fun e => (e.deed, e.performer, e.patient)) =
        [(some adapt.deed, none, some .object)] ∧
      (adapted.referents.filter (·.stampedBy adapt.deed)).map (·.det) = [.target] ∧
      resolved.any (fun e => e.deed == some adapt.deed && e.performer == some .player &&
        e.subjectFits (some .object) true) = true := by
  decide

/-- Aragorn, Company Leader -/
def aragornDistributive : Ability :=
  whenever (does (actor you) (putCounters anyAmount anyKind thisCreature))
    (putCounters 1 thoseCounters
       (targets (upTo 1) (and [isCreature, isOther])))
theorem okAragornDistributive : Ability.check (DRS.ofList []) aragornDistributive = [] := by decide
/-- Captain Marvel, Apex Avenger -/
def captainMarvelSameKinds : Ability :=
  whenever (does (actor you) (putCounters anyAmount anyKind (a (and [isCreature, isOther]))))
    (if_ := some (not (is it kree)))
    (may (putCounters thatMuch thoseCounters thisCreature))
theorem okCaptainMarvelSameKinds : Ability.check (DRS.ofList []) captainMarvelSameKinds = [] := by decide
/-- Denry Klin, Editor in Chief -/
def denryKlinSameKinds : Ability :=
  (match (cardNamed "Denry Klin, Editor in Chief").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons part _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okDenryKlinSameKinds : Ability.check (DRS.ofList []) denryKlinSameKinds = [] := by decide

/-- Me, the Immortal -/
def meTheImmortalCounterMenu : Ability :=
  at_ (beginningOfYour .combat)
    (putCounters 1
      (chosenKind [p1p1Counter, firstStrikeCounter, vigilanceCounter, menaceCounter])
      thisCreature)
theorem okMeTheImmortalCounterMenu : Ability.check (DRS.ofList []) meTheImmortalCounterMenu = [] := by decide

/-- Runadi, Behemoth Caller -/
def runadiBehemothCaller : Ability :=
  static (abilityGrant
    (allOf (and [ isCreature, youControl,
                   compare .atLeast 3 (axes := [.counter p1p1Counter]) ]))
    haste)
theorem okRunadiBehemothCaller : Ability.check (DRS.ofList []) runadiBehemothCaller = [] := by decide
/-- Boon of Safety -/
def boonOfSafetyPut : Action :=
  putCounters 1 shieldCounter (target isCreature)
theorem okBoonOfSafetyPut : Action.check (DRS.ofList []) boonOfSafetyPut = [] := by decide
/-- Vivien's Talent and Teferi's Talent -/
def talentLoyaltyPut : Action :=
  putCounters 1 loyaltyCounter (enchanted
                                                                  planeswalker)
theorem okTalentLoyaltyPut : Action.check (DRS.ofList []) talentLoyaltyPut = [] := by decide
/-- Simic Fluxmage -/
def simicFluxmageMove : Action :=
  moveCounters 1 thisCreature (target isCreature) p1p1Counter
theorem okSimicFluxmageMove : Action.check (DRS.ofList []) simicFluxmageMove = [] := by decide
/-- Rikku, Resourceful Guardian -/
def rikkuStealMove : Action :=
  moveCounters 1 (target (and [isCreature, anOpponentControls])) (target
                                                                                (and
                                                                                   [isCreature,
                                                                                    youControl]))
theorem okRikkuStealMove : Action.check (DRS.ofList []) rikkuStealMove = [] := by decide
/-- Littjara Mirrorlake -/
def littjaraMirrorlakeCopy : Action :=
  create 1
    (copyOf (target (and [isCreature, youControl]))
      [withEntryCounters 1 p1p1Counter .additional])
    []
theorem okLittjaraMirrorlakeCopy : Action.check (DRS.ofList []) littjaraMirrorlakeCopy = [] := by decide

/-- Novijen Sages -/
def novijenSagesDraw : Ability :=
  activated
    (compound
      [ [generic 1],
        (removeCounters (exactly 2) p1p1Counter (among (allOf (and [isCreature, youControl])))) ])
    (draw 1)
theorem okNovijenSagesDraw : Ability.check (DRS.ofList []) novijenSagesDraw = [] := by decide
/-- Cyclone -/
def cycloneUpkeepPayment : Ability :=
  at_ (beginningOfYour .upkeep)
    (sequentially
      [ putCounters 1 windCounter thisEnchantment,
        may (pay (scaled [pip .green] (times 1 (countersOn windCounter thisEnchantment))) .once)
          (instruct thisEnchantment
            (dealDamage (countersOn windCounter thisEnchantment)
              (each (or [isCreature, isPlayer]))))
          (sacrifice thisEnchantment) ])
theorem okCycloneUpkeepPayment : Ability.check (DRS.ofList []) cycloneUpkeepPayment = [] := by decide

def curatorBeastieLine : StaticSpec :=
  entersWithAdditionalCounters (allOf (and [isCreature, youControl, isColorless]))
    2 p1p1Counter
theorem okCuratorBeastieLine : StaticSpec.check (DRS.ofList []) curatorBeastieLine = [] := by decide
def renataLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (and [isCreature, youControl, isOther])) 1
    p1p1Counter
theorem okRenataLine : StaticSpec.check (DRS.ofList []) renataLine = [] := by decide
def entDraughtBasinAbility : Ability :=
  activatedOnlyDuring (compound [mana [.variable], tapSymbol])
    (putCounters 1 p1p1Counter
      (target (and [isCreature, hasPower .eq (letter .x)])))
    asSorcery
theorem okEntDraughtBasinAbility : Ability.check (DRS.ofList []) entDraughtBasinAbility = [] := by decide

def coverOfWinterPut : Ability :=
  activated (mana [.snow]) (putCounters 1 ageCounter thisEnchantment)
theorem okCoverOfWinterPut : Ability.check (DRS.ofList []) coverOfWinterPut = [] := by decide

/-- Vivien's Talent -/
def viviensTalentTrigger : Ability :=
  whenever (does (a (and [isNontoken, and [isCreature, youControl]])) enter)
    (putCounters 1 loyaltyCounter (enchanted planeswalker))
theorem okViviensTalentTrigger : Ability.check (DRS.ofList []) viviensTalentTrigger = [] := by decide
/-- Bioessence Hydra -/
def bioessenceHydraTrigger : Ability :=
  whenever
    (does any (putCounters anyAmount loyaltyCounter
      (allOf (and [isPlaneswalker, youControl]))))
    (putCounters thatMuch p1p1Counter thisCreature)
theorem okBioessenceHydraTrigger : Ability.check (DRS.ofList []) bioessenceHydraTrigger = [] := by decide
/-- Vraska, Betrayal's Sting -/
def vraskaBetrayalsStingUltimate : Action :=
  doIf (compareAmt (countersOn poison (target isPlayer)) .less 9)
    (putCounters theDifference poison they)
theorem okVraskaBetrayalsStingUltimate :
    Action.check (DRS.ofList []) vraskaBetrayalsStingUltimate = [] := by decide
/-- Vexing Puzzlebox -/
def vexingPuzzleboxCounters : Ability :=
  whenever (does you (rollDice anyAmount .anyDie))
    (putCounters (theOutcome .rollResult) chargeCounter thisArtifact)
theorem okVexingPuzzleboxCounters : Ability.check (DRS.ofList []) vexingPuzzleboxCounters = [] := by decide
def spaceFamilyGoblinsonRoll : Ability :=
  whenever (does you (rollDice 1 .anyDie))
    (putCounters 1 p1p1Counter thisCreature)
theorem okSpaceFamilyGoblinsonRoll : Ability.check (DRS.ofList []) spaceFamilyGoblinsonRoll = [] := by decide
/-- Atomwheel Acrobats -/
def atomwheelAcrobatsRoll : Ability :=
  whenever (does you (rollDice (watch := resultIn (oneThrough 2))))
    (putCounters thatMuch p1p1Counter thisCreature)
theorem okAtomwheelAcrobatsRoll : Ability.check (DRS.ofList []) atomwheelAcrobatsRoll = [] := by decide
/-- Resolute Veggiesaur -/
def resoluteVeggiesaurThirdDie : Ability :=
  whenever (nthOccurrence (.nth 3) (does you (rollDice 1 .anyDie)) (some .turn))
    (putCounters 1 p1p1Counter thisCreature)
theorem okResoluteVeggiesaurThirdDie : Ability.check (DRS.ofList []) resoluteVeggiesaurThirdDie = [] := by decide
/-- Run the Play (Striding Shotcaller's other half) -/
def runThePlayCounters : Action :=
  putCounters 1 p1p1Counter
    (eachOf (targets (upToOf (letter .x)) isCreature))
theorem okRunThePlayCounters : Action.check (DRS.ofList [letterB .x]) runThePlayCounters = [] := by decide
/-- Font of Agonies -/
def fontOfAgoniesTrigger : Ability :=
  whenever (does you (payLife anyAmount))
    (putCounters thatMuch bloodCounter thisEnchantment)
theorem okFontOfAgoniesTrigger : Ability.check (DRS.ofList []) fontOfAgoniesTrigger = [] := by decide
/-- Heart of Kiran -/
def heartOfKiranCrewAltCost : Ability :=
  static (altCost (allOf (and [abilityHead (.keyword crew), abilityOf this_]))
       (perform
          (removeCounters (exactly 1) loyaltyCounter
             (a (and [isPlaneswalker, youControl])))))
theorem okHeartOfKiranCrewAltCost : Ability.check (DRS.ofList []) heartOfKiranCrewAltCost = [] := by decide
/-- Tainted Adversary -/
def taintedAdversaryOffer : Ability :=
  when (does thisCreature enter)
    (offerWhen (pay [generic 2, pip .black] .anyNumberOfTimes)
       (putCounters thatMuch p1p1Counter thisCreature))
theorem okTaintedAdversaryOffer : Ability.check (DRS.ofList []) taintedAdversaryOffer = [] := by decide
/-- Korvold, Gleeful Glutton -/
def korvoldCombatTrigger : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
    (sequentially
      [ putCounters (letter .x) p1p1Counter thisCreature,
        draw (letter .x),
        define .x (distinctCount .permanentType (allOf (cardIn yourGraveyard))) ])
theorem okKorvoldCombatTrigger : Ability.check (DRS.ofList []) korvoldCombatTrigger = [] := by decide
/-- Mirelurk Queen -/
def mirelurkQueenTrigger : Ability :=
  whenever
    (does any (millCards (counted (orMore 1) (and [non isLand, isCard, in_ library]))))
    (sequentially [draw 1, putCounters 1 p1p1Counter
        thisCreature]) (limit := oncePerTurn)
theorem okMirelurkQueenTrigger : Ability.check (DRS.ofList []) mirelurkQueenTrigger = [] := by decide

/-- Contractual Safeguard -/
def contractualSafeguardPass : Action :=
  sequentially
    [ choose (a (counterKindOn (a (and [isCreature, youControl])))),
      putCounters 1 thatKind (each (and [isCreature, youControl, isOther])) ]
theorem okContractualSafeguardPass : Action.check (DRS.ofList []) contractualSafeguardPass = [] := by decide

/-- Nazgûl -/
def nazgulRingTrigger : Ability :=
  whenever (does you theRingTemptsYou)
    (putCounters 1 p1p1Counter
      (each (and [wraith, youControl])))
theorem okNazgulRingTrigger : Ability.check (DRS.ofList []) nazgulRingTrigger = [] := by decide

def agentOfTheShadowThievesGrantedTrigger : Ability :=
  whenever (does thisCreature (attack (a isPlayer))) (if_ := some (not
                                                                          (exists_
                                                                             (and [ isOpponent,
                                 compare .greater (lifeTotalOf (that player))
                                     (axes := [.playerStat .lifeTotal]) ]))))
                                     (putCounters 1 p1p1Counter thisCreature)
theorem okAgentOfTheShadowThievesGrantedTrigger :
    Ability.check (DRS.ofList []) agentOfTheShadowThievesGrantedTrigger = [] := by decide

/-- Chamber Sentry -/
def chamberSentryDamage : Ability :=
  activated
    (compound
      [ mana [.variable], tapSymbol,
        (removeCounters (exactlyOf (letter .x)) p1p1Counter thisCreature) ])
    (instruct this_ (dealDamage (letter .x) (target isAnyTarget)))
theorem okChamberSentryDamage : Ability.check (DRS.ofList []) chamberSentryDamage = [] := by decide

/-- Aether Chaser -/
def aetherChaserEnergy : Ability :=
  when (does thisCreature enter) (putCounters 2 energy you)
theorem okAetherChaserEnergy : Ability.check (DRS.ofList []) aetherChaserEnergy = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossEntersOiled : StaticSpec :=
  entersWithCounters thisCreature 4 oilCounter
theorem okArchfiendOfTheDrossEntersOiled :
    StaticSpec.check (DRS.ofList []) archfiendOfTheDrossEntersOiled = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossOilUpkeep : Ability :=
  at_ (beginningOfYour .upkeep)
    (removeCounters (exactly 1) oilCounter thisCreature)
theorem okArchfiendOfTheDrossOilUpkeep : Ability.check (DRS.ofList []) archfiendOfTheDrossOilUpkeep = [] := by
  decide
def neurokTransmuter : Action :=
  become (target isCreature) { characteristics := { types := [.artifact] } } untilEndOfTurn
theorem okNeurokTransmuter : Action.check (DRS.ofList []) neurokTransmuter = [] := by decide
def syphonMind : Action :=
  sequentially
    [ instruct (each isOtherPlayer) (discard (a (cardIn hand))),
      doForEach (each (and [isCard, was (discard which) thisWay])) (draw 1) ]
theorem okSyphonMind : Action.check (DRS.ofList []) syphonMind = [] := by decide
def peek : Action := sequentially [lookAtHandOf (target isPlayer), draw 1]
theorem okPeek : Action.check (DRS.ofList []) peek = [] := by decide
/-- Bumi, King of Three Trials -/
def bumiScryMode : Action := instruct (target isPlayer) (scry 3)
theorem okBumiScryMode : Action.check (DRS.ofList []) bumiScryMode = [] := by decide
/-- Final Act -/
def finalActCounterMode : Action := loseAllCounters none (agent := eachOpponent)
theorem okFinalActCounterMode : Action.check (DRS.ofList []) finalActCounterMode = [] := by decide
def leeches : Action :=
  loseAllCounters poison (agent := target isPlayer)
theorem okLeeches : Action.check (DRS.ofList []) leeches = [] := by decide

end Semantics.Cards
