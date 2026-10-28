import Semantics.Cards
import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsTrigger

/-!
# Semantics.Cards.Keyword

Port of `idris/src/Experimental/Cards/Keyword.idr`: the printed cards of the Keyword family
(keyword abilities, their parameters and grants) and the bench items beside them.

Battle Squadron prints a `*/*` box: those slots are `none` under the printed-star ruling.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def jump : Action := gain (target isCreature) flying untilEndOfTurn
theorem okJump : Action.check (DRS.ofList []) jump = [] := by decide
def gabrielAngelfire : Action :=
  sequentially
    [ choose (a (qualityFrom .ability
        (abilitiesAmong [flying, firstStrike, trample, .theWith rampage 3]))),
      gain thisCreature (thatAbility .theChoice) (untilEvent (beginningOf .upkeep you nextTime)) ]
theorem okGabrielAngelfire : Action.check (DRS.ofList []) gabrielAngelfire = [] := by decide
def builtToSmash : Action :=
  sequentially
    [ get (target (and [isCreature, isAttacking])) (up 3) (up 3) untilEndOfTurn,
      doIf (is it (and [isArtifact, isCreature]))
        (gain it trample untilEndOfTurn) ]
theorem okBuiltToSmash : Action.check (DRS.ofList []) builtToSmash = [] := by decide

/-- A flying Thopter token. -/
def thopterToken : CharacteristicBundle :=
  { characteristics :=
    { types := [.artifact, .creature], subtypes := [thopter],
      text := [flying], power := stat 1, toughness := stat 1 } }

def aviationPioneer : Action := create 1 thopterToken
theorem okAviationPioneer : Action.check (DRS.ofList []) aviationPioneer = [] := by decide
def fireNavyTrebuchet : Action :=
  createTappedAttacking 1
    { characteristics :=
      { name := "Ballistic Boulder", types := [.artifact, .creature],
        subtypes := [construct], text := [flying],
        power := stat 2, toughness := stat 1 } }
theorem okFireNavyTrebuchet : Action.check (DRS.ofList []) fireNavyTrebuchet = [] := by decide

/-- Pym Particles -/
def pymParticlesGrant : Action :=
  sequentially
    [ gain (target isCreature) vigilance untilEndOfTurn,
      establishThroughout (cant any (block (that creature))) thisTurn ]
theorem okPymParticlesGrant : Action.check (DRS.ofList []) pymParticlesGrant = [] := by decide
def bladebrand : Action := gain (target isCreature) deathtouch untilEndOfTurn
theorem okBladebrand : Action.check (DRS.ofList []) bladebrand = [] := by decide
def criticalHit : Action :=
  gain (target isCreature) doubleStrike untilEndOfTurn
theorem okCriticalHit : Action.check (DRS.ofList []) criticalHit = [] := by decide
def lightningBlow : Action :=
  gain (target isCreature) firstStrike untilEndOfTurn
theorem okLightningBlow : Action.check (DRS.ofList []) lightningBlow = [] := by decide
def deathByDragons : Action :=
  instruct (each (and [isPlayer, otherThan (target isPlayer)])) (create 1 (written { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [dragon],
          text := [flying], power := stat 5, toughness := stat 5 } }))
theorem okDeathByDragons : Action.check (DRS.ofList []) deathByDragons = [] := by decide

def firesongAndSunspeaker : Ability :=
  static (abilityGrant
    (allOf (and [hasColor .red, isInstantOrSorcery, isSpell, youControl]))
    lifelink)
theorem okFiresongAndSunspeaker : Ability.check (DRS.ofList []) firesongAndSunspeaker = [] := by decide
def briaRiptideRogue : Ability :=
  static (abilityGrant (allOf (and [isCreature, youControl, isOther])) prowess)
theorem okBriaRiptideRogue : Ability.check (DRS.ofList []) briaRiptideRogue = [] := by decide
def narsetEnlightenedExile : Ability :=
  static (abilityGrant (allOf (and [isCreature, youControl])) prowess)
theorem okNarsetEnlightenedExile : Ability.check (DRS.ofList []) narsetEnlightenedExile = [] := by decide
def pontiffOfBlight : Ability :=
  static (abilityGrant (allOf (and [isCreature, youControl, isOther])) extort)
theorem okPontiffOfBlight : Ability.check (DRS.ofList []) pontiffOfBlight = [] := by decide

/-- Blade of the Oni -/
def bladeOfTheOniStatic : StaticSpec :=
  conjunction [ getsBase (equipped creature) 5 5,
      abilityGrant it menace,
      qualityChange it .adds
        (bundle { characteristics := { colors := [.black], subtypes := [demon] } }) ]
theorem okBladeOfTheOniStatic : StaticSpec.check (DRS.ofList []) bladeOfTheOniStatic = [] := by decide

def castCreatureSpellsFromTop : StaticSpec :=
  (match (cardNamed "Garruk's Horde").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons (Ability.static part) _))) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okCastCreatureSpellsFromTop : StaticSpec.check (DRS.ofList []) castCreatureSpellsFromTop = [] := by decide

def tayamLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (and [isCreature, youControl, isOther])) 1
    vigilanceCounter
theorem okTayamLine : StaticSpec.check (DRS.ofList []) tayamLine = [] := by decide

/-- Kang the Conqueror -/
def kangPowerUpLock : StaticSpec :=
  cant any (activate (allOf (abilityHead (.keyword powerUp))))
theorem okKangPowerUpLock : StaticSpec.check (DRS.ofList []) kangPowerUpLock = [] := by decide

def stormOfSouls : Action :=
  sequentially
    [ returnToBattlefield (allOf (and [isCreature, in_ yourGraveyard])) (from_ := yourGraveyard),
      become them
        { characteristics :=
          { subtypes := [spirit], text := [flying],
            power := stat 1, toughness := stat 1 } },
      exile this_ ]
theorem okStormOfSouls : Action.check (DRS.ofList []) stormOfSouls = [] := by decide

/-- Hate Mirage's middle two sentences -/
def hateMirageTokens : Action :=
  sequentially
    [ doForEach (targets (upTo 2) (and [isCreature, youDontControl]))
        (create 1 (copyOf it) []),
      gain thoseTokens haste ]
theorem okHateMirageTokens : Action.check (DRS.ofList []) hateMirageTokens = [] := by decide
def descentOfTheDragons : Action :=
  sequentially
    [ destroy (targets anyNumber isCreature),
      doForEach (each (and [isCreature, was (destroy which) thisWay]))
        (instruct (controllerOf it) (create 1 (written { characteristics :=
              { colors := [.red], types := [.creature], subtypes := [dragon],
                text := [flying], power := stat 4, toughness := stat 4 } }))) ]
theorem okDescentOfTheDragons : Action.check (DRS.ofList []) descentOfTheDragons = [] := by decide
/-- Brimaz, King of Oreskos -/
def brimazAttackToken : Ability :=
  whenever (does thisCreature attack)
    (create 1
      (written
        { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [cat, soldier],
            text := [vigilance], power := stat 1, toughness := stat 1 } })
      [entersAttacking none])
theorem okBrimazAttackToken : Ability.check (DRS.ofList []) brimazAttackToken = [] := by decide
/-- Sedris, the Traitor King -/
def sedrisTheTraitorKing : Ability :=
  static (abilityGrant (each (and [isCreature, in_ yourGraveyard]))
    (unearth [generic 2, pip .black]))
theorem okSedrisTheTraitorKing : Ability.check (DRS.ofList []) sedrisTheTraitorKing = [] := by decide
/-- Grixis -/
def grixis : Ability :=
  static (abilityGrant
    (allOf (and [ andOr [hasColor .blue, hasColor .black, hasColor .red], isCreature,
                   in_ yourGraveyard ]))
    (unearth itsManaCost))
theorem okGrixis : Ability.check (DRS.ofList []) grixis = [] := by decide
/-- Dralnu, Lich Lord -/
def dralnuLichLord : Action :=
  gain (target (and [isInstantOrSorcery, in_ yourGraveyard]))
    (flashback itsManaCost) untilEndOfTurn
theorem okDralnuLichLord : Action.check (DRS.ofList []) dralnuLichLord = [] := by decide
/-- Dregscape Zombie -/
def dregscapeZombie : Ability := unearth [pip .black]
theorem okDregscapeZombie : Ability.check (DRS.ofList []) dregscapeZombie = [] := by decide
/-- Think Twice -/
def thinkTwice : Ability := flashback [generic 2, pip .blue]
theorem okThinkTwice : Ability.check (DRS.ofList []) thinkTwice = [] := by decide
/-- Greater Mossdog -/
def greaterMossdog : Ability := dredge 3
theorem okGreaterMossdog : Ability.check (DRS.ofList []) greaterMossdog = [] := by decide
/-- Raven's Crime -/
def ravensCrime : Ability := retrace
theorem okRavensCrime : Ability.check (DRS.ofList []) ravensCrime = [] := by decide
/-- Barkhide Mauler -/
def barkhideMauler : Ability := cycling [generic 2]
theorem okBarkhideMauler : Ability.check (DRS.ofList []) barkhideMauler = [] := by decide
/-- Ninja of the New Moon -/
def ninjaOfTheNewMoon : Ability := ninjutsu [generic 3, pip .black]
theorem okNinjaOfTheNewMoon : Ability.check (DRS.ofList []) ninjaOfTheNewMoon = [] := by decide
/-- Thunderous Wrath -/
def thunderousWrath : Ability := miracle [pip .red]
theorem okThunderousWrath : Ability.check (DRS.ofList []) thunderousWrath = [] := by decide
/-- Bygone Colossus -/
def bygoneColossus : Ability := warp [generic 3]
theorem okBygoneColossus : Ability.check (DRS.ofList []) bygoneColossus = [] := by decide

/-- Knight of Grace — "Hexproof from black" [CR#702.11d] -/
def knightOfGraceHexproofFromBlack : Ability := hexproofFrom (hasColor .black)
theorem okKnightOfGraceHexproofFromBlack :
    Ability.check (DRS.ofList []) knightOfGraceHexproofFromBlack = [] := by decide
/-- Eternal Dragon — "Plainscycling {2}" [CR#702.29e] -/
def eternalDragonPlainscycling : Ability :=
  typecycling plains [generic 2]
theorem okEternalDragonPlainscycling : Ability.check (DRS.ofList []) eternalDragonPlainscycling = [] := by decide
/-- Nezumi Ronin -/
def nezumiRonin : Ability := bushido 1
theorem okNezumiRonin : Ability.check (DRS.ofList []) nezumiRonin = [] := by decide
/-- Dragonlord Ojutai -/
def dragonlordOjutaiHexproof : Ability :=
  static (onlyWhile (abilityGrant thisCreature hexproof) (is it isUntapped))
theorem okDragonlordOjutaiHexproof : Ability.check (DRS.ofList []) dragonlordOjutaiHexproof = [] := by decide
/-- Monoxa, Midway Manager -/
def monoxaRollTrigger : Ability :=
  whenever (does you (rollDice (watch := resultIn (orMore 3))))
    (sequentially
      [ gain thisCreature firstStrike untilEndOfTurn,
        doIf (compareAmt (theOutcome .rollResult) .atLeast 4)
          (gain it menace untilEndOfTurn),
        doIf (compareAmt (theOutcome .rollResult) .atLeast 5)
          (gain it lifelink untilEndOfTurn) ])
theorem okMonoxaRollTrigger : Ability.check (DRS.ofList []) monoxaRollTrigger = [] := by decide
/-- Celebr-8000 -/
def celebr8000Doubles : Action :=
  sequentially
    [ rollDice 2 (.sides 6),
      doIf rolledDoubles (gain thisCreature doubleStrike untilEndOfTurn) ]
theorem okCelebr8000Doubles : Action.check (DRS.ofList []) celebr8000Doubles = [] := by decide

/-- New Perspectives: "As long as you have seven or more cards in hand, you may pay {0} rather
than pay cycling costs." Cycling is an activated ability [CR#702.29a], and an alternative cost
replaces an activation cost as it replaces a spell's mana cost [CR#118.9,602.2b]. -/
def newPerspectivesCyclingAltCost : Ability :=
  (match (cardNamed "New Perspectives").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons part _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okNewPerspectivesCyclingAltCost :
    Ability.check (DRS.ofList []) newPerspectivesCyclingAltCost = [] := by decide
/-- Whether an ability is an alternative cost that applies only "as long as" a condition holds. -/
def altCostAsLongAs : Ability → Bool
  | .static (.conditional (.altCost ..) _ .asLongAs) => true
  | _ => false
/-- The condition-less spelling drops the printed "as long as you have seven or more cards in
hand": the card's alternative cost sits under a `conditional … .asLongAs`, which that spelling
lacks. -/
theorem badNewPerspectivesWithoutCondition :
    altCostAsLongAs newPerspectivesCyclingAltCost = true ∧
      altCostAsLongAs (static (altCost (allOf (abilityHead (.keyword cycling))) [generic 0]))
        = false := by decide

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
/-- Fumiko the Lowblood -/
def fumikoBushidoX : Ability :=
  static (conjunction [ abilityGrant thisCreature (bushido (letter .x)),
      letterDefinition .x (countOf isAttacking) ])
theorem okFumikoBushidoX : Ability.check (DRS.ofList []) fumikoBushidoX = [] := by decide

/-- Tourach, Dread Cantor -/
def tourachDiscardTrigger : Ability :=
  (match (cardNamed "Tourach, Dread Cantor").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons part _))) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okTourachDiscardTrigger : Ability.check (DRS.ofList []) tourachDiscardTrigger = [] := by decide

/-- Soul of Emancipation -/
def soulOfEmancipation : Ability :=
  when (does thisCreature enter)
    (sequentially
      [ destroy (targets (upTo 3) (and [isPermanent, non isLand, isOther])),
        doForEach (those permanent)
          (instruct (controllerOf (that permanent)) (create 1 (written { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [angel],
                  text := [flying], power := stat 3, toughness := stat 3 } }))) ])
theorem okSoulOfEmancipation : Ability.check (DRS.ofList []) soulOfEmancipation = [] := by decide

/-- Talrand's Invocation -/
def talrandsInvocation : Action :=
  create 2 (written
    { characteristics :=
      { colors := [.blue], types := [.creature], subtypes := [drake],
        text := [flying], power := stat 2, toughness := stat 2 } })
theorem okTalrandsInvocation : Action.check (DRS.ofList []) talrandsInvocation = [] := by decide

/-- Harried Dronesmith -/
def harriedDronesmithToken : Action :=
  sequentially [create 1 thopterToken, gainHaste thatToken untilEndOfTurn]
theorem okHarriedDronesmithToken : Action.check (DRS.ofList []) harriedDronesmithToken = [] := by decide

/-- Veiling Oddity -/
def veilingOddityLine : Ability :=
  (match (cardNamed "Veiling Oddity").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons part _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okVeilingOddityLine : Ability.check (DRS.ofList []) veilingOddityLine = [] := by decide

/-- Darkblade Agent -/
def darkbladeAgentGrants : Ability :=
  static (onlyWhile (conjunction [ abilityGrant thisCreature deathtouch,
        abilityGrant thisCreature
          (whenever
             (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat))) (draw
                                                                                                  1)) ]) (did you (surveil anyAmount) soFarThisTurn))
theorem okDarkbladeAgentGrants : Ability.check (DRS.ofList []) darkbladeAgentGrants = [] := by decide
/-- Frenzied Gorespawn -/
def frenziedGorespawnMenaceTrigger : Ability :=
  whenever (does (counted (orMore 1) isCreature) (attack anOpponent))
      (gain (those creature) menace untilEndOfTurn)
theorem okFrenziedGorespawnMenaceTrigger :
    Ability.check (DRS.ofList []) frenziedGorespawnMenaceTrigger = [] := by decide
/-- Pir, Imaginative Rascal's replacement -/
def pirDistributive : Ability :=
  (match (cardNamed "Pir, Imaginative Rascal").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons part _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okPirDistributive : Ability.check (DRS.ofList []) pirDistributive = [] := by decide

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
        (StaticSpec.characteristicChange _ (List.cons (CharacteristicEdit.addedSelectionOf _
        (AbilitySelection.specified (List.cons (AbilityLost.term part) _))) _)) _ _) _))))))))) _))
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
/-- Tolaria, Shelkin Brownie -/
def bandsWithOtherAbilities : KeywordTerm := (match (cardNamed "Shelkin Brownie").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.activated _ (Action.establish (StaticSpec.characteristicChange _
        (List.cons (CharacteristicEdit.removedAbilities (AbilitySelection.specified (List.cons
        (AbilityLost.term part) _))) _)) _) _ _ _ _) _) => some part
      | _ => none
    | _ => none).get (by decide)
def protectionFromAnyColor : KeywordTerm := (match (cardNamed "Escaped Shapeshifter").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.static (StaticSpec.conjunction _ (List.cons _ (List.cons _ (List.cons _
        (List.cons (StaticSpec.conditional (StaticSpec.characteristicChange _ (List.cons
        (CharacteristicEdit.addedSelectionOf _ (AbilitySelection.specified (List.cons
        (AbilityLost.term part) _))) _)) _ _) _)))))) _) => some part
      | _ => none
    | _ => none).get (by decide)

/-- Commander's Plate's equip lines -/
def commandersPlateEquip : List Ability :=
  [ equipQuality (hasDesignation commander) [generic 3],
    equip [generic 5] ]
theorem okCommandersPlateEquip : Ability.checkText (DRS.ofList []) commandersPlateEquip = [] := by decide
/-- Luxior, Giada's Gift's equip lines -/
def luxiorEquipLines : List Ability :=
  [ equipQuality isPlaneswalker [generic 1],
    equip [generic 3] ]
theorem okLuxiorEquipLines : Ability.checkText (DRS.ofList []) luxiorEquipLines = [] := by decide

/-- Bedrock Tortoise -/
def bedrockTortoiseWindow : StaticSpec :=
  partScope .turn you (abilityGrant (allOf (and [isCreature, youControl])) hexproof)
theorem okBedrockTortoiseWindow : StaticSpec.check (DRS.ofList []) bedrockTortoiseWindow = [] := by decide

/-- Marit Lage -/
def maritLage : CharacteristicBundle :=
  (match (cardNamed "Dark Depths").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons (Ability.triggered _ _ _ _ _ _ _
        (Action.withContinuation _ _ (Option.some (Action.ownScope _ (Action.createObject _
        (CreationSpec.token (TokenSpec.inCaller (TokenSpec.written part)) _) _))) _)) _))) => some
        part
      | _ => none
    | _ => none).get (by decide)

def giantGrowth : Action :=
  get (target isCreature) (up 3) (up 3) untilEndOfTurn
theorem okGiantGrowth : Action.check (DRS.ofList []) giantGrowth = [] := by decide
def amassZombiesTwo : Action := amass zombie 2
theorem okAmassZombiesTwo : Action.check (DRS.ofList []) amassZombiesTwo = [] := by decide
/-- Avarice Totem -/
def avariceTotemExchange : Action :=
  exchangeControl thisArtifact (target (and [isPermanent, non isLand]))
theorem okAvariceTotemExchange : Action.check (DRS.ofList []) avariceTotemExchange = [] := by decide
/-- Arcanum Wings -- aura swap [CR#702.65a] -/
def arcanumWingsAuraSwap : Ability :=
  auraSwap [generic 2, pip .blue]
theorem okArcanumWingsAuraSwap : Ability.check (DRS.ofList []) arcanumWingsAuraSwap = [] := by decide
/-- Tovolar, Dire Overlord -/
def tovolarNightfall : Action := setGameDesignation night
theorem okTovolarNightfall : Action.check (DRS.ofList []) tovolarNightfall = [] := by decide
/-- Spin into Myth -/
def spinIntoMyth : Action :=
  sequentially [putInto (target isCreature) onTop, fateseal 2]
theorem okSpinIntoMyth : Action.check (DRS.ofList []) spinIntoMyth = [] := by decide
/-- Pure // Simple -/
def pureHalf : Action := destroy (target (and [isPermanent, isMulticolored]))
theorem okPureHalf : Action.check (DRS.ofList []) pureHalf = [] := by decide
/-- Korlash -/
def grandeurDiscardCost : Cost :=
  (discard (a (and [named "Korlash, Heir to Blackblade", isOther, in_ yourHand])))
theorem okGrandeurDiscardCost : Cost.check (DRS.ofList []) grandeurDiscardCost = [] := by decide
/-- Gyruda, Doom of Depths -/
def gyrudaCompanion : Ability := companion (everyCardIs isCard (manaValueParity .even))
theorem okGyrudaCompanion : Ability.check (DRS.ofList []) gyrudaCompanion = [] := by decide
/-- Jegantha, the Wellspring -/
def jeganthaCompanion : Ability := companion (noCardIs isCard repeatedManaSymbol)
theorem okJeganthaCompanion : Ability.check (DRS.ofList []) jeganthaCompanion = [] := by decide
/-- Kaheera, the Orphanguard -/
def kaheeraCompanion : Ability :=
  companion
    (everyCardIs (and [isCreature, isCard])
      (anyOf [ aCharacteristic cat,
                aCharacteristic elemental,
                aCharacteristic nightmare,
                aCharacteristic dinosaur,
                aCharacteristic beast ]))
theorem okKaheeraCompanion : Ability.check (DRS.ofList []) kaheeraCompanion = [] := by decide
/-- Keruga, the Macrosage -/
def kerugaCompanion : Ability :=
  companion
    (everyCardIs isCard
      (anyOf [ aCharacteristic (hasManaValue .atLeast 3),
                aCharacteristic isLand ]))
theorem okKerugaCompanion : Ability.check (DRS.ofList []) kerugaCompanion = [] := by decide
/-- Lurrus of the Dream-Den -/
def lurrusCompanion : Ability :=
  companion
    (everyCardIs isPermanentCard (aCharacteristic (hasManaValue .atMost 2)))
theorem okLurrusCompanion : Ability.check (DRS.ofList []) lurrusCompanion = [] := by decide
/-- Lutri, the Spellchaser -/
def lutriCompanion : Ability := companion (groupAgrees
                                             (withDifferentNames
                                                (allOf (and [non isLand, isCard]))))
theorem okLutriCompanion : Ability.check (DRS.ofList []) lutriCompanion = [] := by decide
/-- Obosh, the Preypiercer -/
def oboshCompanion : Ability :=
  companion (everyCardIs isCard (anyOf [manaValueParity .odd, aCharacteristic isLand]))
theorem okOboshCompanion : Ability.check (DRS.ofList []) oboshCompanion = [] := by decide
/-- Umori, the Collector -/
def umoriCompanion : Ability := companion
    (groupAgrees (agree .cardType .share (allOf (and [non isLand, isCard]))))
theorem okUmoriCompanion : Ability.check (DRS.ofList []) umoriCompanion = [] := by decide
/-- Yorion, Sky Nomad -/
def yorionCompanion : Ability := companion (deckSizeOverMinimum 20)
theorem okYorionCompanion : Ability.check (DRS.ofList []) yorionCompanion = [] := by decide
/-- Zirda, the Dawnwaker -/
def zirdaCompanion : Ability :=
  companion (everyCardIs isPermanentCard (hasAbilityOf .anyActivated))
theorem okZirdaCompanion : Ability.check (DRS.ofList []) zirdaCompanion = [] := by decide

end Semantics.Cards
