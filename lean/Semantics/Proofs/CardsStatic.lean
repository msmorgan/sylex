import Semantics.Cards
import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsHelpers

/-!
# Semantics.Cards.Static

Port of `idris/src/Experimental/Cards/Static.idr`: the printed cards of the Static family
(continuous clauses: pumps, definitions, ascriptions, locks, alternative costs, visibility) and
the bench items beside them.

Maro, People of the Woods and Cultivator Colossus print `*` boxes: those slots are `none`
under the printed-star ruling, and their Characteristic-Defining Abilities set them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def forkedBolt : Action :=
  (match (cardNamed "Chandra's Pyrohelix").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ part) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okForkedBolt : Action.check (DRS.ofList []) forkedBolt = [] := by decide
def thoughtReflection : Ability :=
  static (replace (would you (draw 1)) (instead (draw 2)))
theorem okThoughtReflection : Ability.check (DRS.ofList []) thoughtReflection = [] := by decide
def jorKadeen : Ability :=
  static (onlyWhile (getsPt (allOf (and [isCreature, youControl])) (up 3) (up 0))
    (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 3))
theorem okJorKadeen : Ability.check (DRS.ofList []) jorKadeen = [] := by decide
def abandonedOutpost : Ability := static (entersTapped thisLand)
theorem okAbandonedOutpost : Ability.check (DRS.ofList []) abandonedOutpost = [] := by decide
/-- Time Vault -/
def timeVaultLock : Ability := static (doesntUntap thisArtifact you)
theorem okTimeVaultLock : Ability.check (DRS.ofList []) timeVaultLock = [] := by decide
def hymnOfRebirth : Action :=
  (match (cardNamed "Hymn of Rebirth").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ part) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okHymnOfRebirth : Action.check (DRS.ofList []) hymnOfRebirth = [] := by decide
def counterspell : Action := (match (cardNamed "Counterspell").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ part) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okCounterspell : Action.check (DRS.ofList []) counterspell = [] := by decide

def adantoVanguard : Ability :=
  static (onlyWhile (getsPt thisCreature (up 2) (up 0))
    (is thisCreature isAttacking))
theorem okAdantoVanguard : Ability.check (DRS.ofList []) adantoVanguard = [] := by decide
/-- Nowhere to Run -/
def nowhereToRunTargetLine : StaticSpec :=
  can (by_ (allOf (or [isSpell, abilityHead .anyOnStack]))) (target (allOf (and [isCreature, opponentsControl]))) (asThough := holds (is them (not (hasKeyword hexproof))))
theorem okNowhereToRunTargetLine : StaticSpec.check (DRS.ofList []) nowhereToRunTargetLine = [] := by decide
/-- Hithlain Rope -/
def hithlainRopeSacrificeLock : StaticSpec := cant any (sacrifice this_)
theorem okHithlainRopeSacrificeLock : StaticSpec.check (DRS.ofList []) hithlainRopeSacrificeLock = [] := by
  decide
/-- Lich's Mastery -/
def lichsMasteryGate : Ability := static (cant you loseGame)
theorem okLichsMasteryGate : Ability.check (DRS.ofList []) lichsMasteryGate = [] := by decide
def theGoldenThrone : Ability :=
  static (replace (would you loseGame) (instead (sequentially [exile thisArtifact, setLife 1])))
theorem okTheGoldenThrone : Ability.check (DRS.ofList []) theGoldenThrone = [] := by decide
def stunningReversal : Ability :=
  spell (establishThroughout
           (replace (would you loseGame) (instead (sequentially [draw 7, setLife 1]))
              none .nextTimeOnly) thisTurn)
theorem okStunningReversal : Ability.check (DRS.ofList []) stunningReversal = [] := by decide
def pathOfBravery : Ability :=
  static (onlyWhile (getsPt (allOf (and [isCreature, youControl])) (up 1) (up 1))
    (compareAmt (lifeTotalOf you) .atLeast (statOf (.playerStat .startingLifeTotal) you)))
theorem okPathOfBravery : Ability.check (DRS.ofList []) pathOfBravery = [] := by decide

def spontaneousMutation : Ability :=
  static (conjunction [ getsPt (enchanted creature) (down (letter .x)) (down 0),
      letterDefinition .x (countOf (cardIn yourGraveyard)) ])
theorem okSpontaneousMutation : Ability.check (DRS.ofList []) spontaneousMutation = [] := by decide

def scourgeOfTheSkyclaves : Ability :=
  static (ptDefinition thisCreature .bothEach
    (minus 20 (aggregate .max (.playerStat .lifeTotal) isPlayer)))
theorem okScourgeOfTheSkyclaves : Ability.check (DRS.ofList []) scourgeOfTheSkyclaves = [] := by decide
def aettirAndPriwen : Ability :=
  static (conjunction [ getsBase (equipped creature) (letter .x) (letter .x),
      letterDefinition .x (lifeTotalOf you) ])
theorem okAettirAndPriwen : Ability.check (DRS.ofList []) aettirAndPriwen = [] := by decide

def cycleOfLife : Action :=
  establishThroughout
    (getsBase (target (and [isCreature, did you (cast which) soFarThisTurn])) 0 1)
    (untilEvent (beginningOf .upkeep you nextTime))
theorem okCycleOfLife : Action.check (DRS.ofList []) cycleOfLife = [] := by decide

def roilingHorror : Ability :=
  static (ptDefinition thisCreature .bothEach
    (minus (lifeTotalOf you)
      (lifeTotalOf
        (a (and [isOpponent, superlative .max (.playerStat .lifeTotal) isOpponent])))))
theorem okRoilingHorror : Ability.check (DRS.ofList []) roilingHorror = [] := by decide

/-- Wilt-Leaf Liege's last line: "If a spell or ability an opponent controls causes you to discard
this card, put it onto the battlefield instead of putting it into your graveyard." -/
def wiltLeafLiegeDiscardReplacement : Ability :=
  (match (cardNamed "Wilt-Leaf Liege").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons part _))) => some part
      | _ => none
    | _ => none).get (by decide)

/-- The discard has a performer (you) and, orthogonally, a cause (a spell or ability an opponent
controls) [CR#609.1,701.9a], and the replacement checks clean with both. -/
theorem okWiltLeafLiegeEffectCausedDiscard :
    Ability.check (DRS.ofList []) wiltLeafLiegeDiscardReplacement = [] ∧
    (match wiltLeafLiegeDiscardReplacement with
      | .static (.replace (.causedBy (.source _) (.would (.actor .you) _)) ..) => true
      | _ => false) = true := by decide

/-- Master Chef -/
def masterChefGrantedAbility : Ability :=
  static (entersWithAdditionalCounters thisCreature 1 p1p1Counter)
theorem okMasterChefGrantedAbility : Ability.check (DRS.ofList []) masterChefGrantedAbility = [] := by decide

/-- Tahngarth, First Mate -/
def tahngarthAttacksThatJoin : Action := becomeAttacking thisCreature thatJoin
theorem okTahngarthAttacksThatJoin :
    Action.check (nextClause ((stepAction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) tahngarthHeader).1) tahngarthChoosesDefender).1))
      tahngarthAttacksThatJoin = [] := by
  decide

def omenMachineDraw : Ability := static (cant (bare isPlayer) (draw anyAmount))
theorem okOmenMachineDraw : Ability.check (DRS.ofList []) omenMachineDraw = [] := by decide
def solfataraLandLock : Action :=
  establishThroughout (cant (target isPlayer) (play any)) thisTurn
theorem okSolfataraLandLock : Action.check (DRS.ofList []) solfataraLandLock = [] := by decide

/-- Volrath's Laboratory -/
def volrathsLaboratoryChoice : StaticSpec :=
  (match (cardNamed "Volrath's Laboratory").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.static part) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okVolrathsLaboratoryChoice : StaticSpec.check (DRS.ofList []) volrathsLaboratoryChoice = [] := by decide
/-- Call to Arms -/
def callToArmsChoice : StaticSpec :=
  conjunction [ entersChoosing thisEnchantment .color,
      entersChoosingPlayer thisEnchantment anOpponent ]
theorem okCallToArmsChoice : StaticSpec.check (DRS.ofList []) callToArmsChoice = [] := by decide

/-- Celestial Dawn -/
def celestialDawnAscriptions : List Ability :=
  [ static (qualityChange (allOf (and [isLand, youControl])) .sets
      (bundle { characteristics := { subtypes := [plains] } })),
    static (qualityChange (and [
              allOf (and [isPermanent, non isLand, youControl]),
              allOf (and [isSpell, youControl]),
              allOf (and [isCard, non isLand, youOwn, not (in_ battlefield)]) ]) .sets
        (colored (.some [.white]))) ]
theorem okCelestialDawnAscriptions : Ability.checkText (DRS.ofList []) celestialDawnAscriptions = [] := by decide

/-- Booby Trap -/
def boobyTrapNameChoice : StaticSpec :=
  entersChoosingFrom thisArtifact .cardName (nameOfCard (not (and [hasSupertype .basic, isLand])))
theorem okBoobyTrapNameChoice : StaticSpec.check (DRS.ofList []) boobyTrapNameChoice = [] := by decide

/-- Nameless Inversion -/
def namelessInversionBody : Action :=
  establishFor (target isCreature)
    [getsPt it (up 3) (down 3), qualityChange it .loses (everyTypeOf .creature)]
    untilEndOfTurn
theorem okNamelessInversionBody : Action.check (DRS.ofList []) namelessInversionBody = [] := by decide
/-- Ego Erasure -/
def egoErasureBody : Action :=
  establishFor (allOf (and [isCreature, controlledBy (target isPlayer)]))
    [getsPt them (down 2) (up 0), qualityChange them .loses (everyTypeOf .creature)]
    untilEndOfTurn
theorem okEgoErasureBody : Action.check (DRS.ofList []) egoErasureBody = [] := by decide
/-- Lithoform Blight -/
def lithoformBlightLoss : StaticSpec :=
  conjunction [ qualityChange (enchanted land) .loses (everyTypeOf .land),
      allAbilityLoss it ]
theorem okLithoformBlightLoss : StaticSpec.check (DRS.ofList []) lithoformBlightLoss = [] := by decide

def theLadyOfOtariaLine : StaticSpec :=
  altCost this_
    (tap
      (counted (exactly 3)
        (and [ isCreature, dwarf, youControl,
                isUntapped ])))
theorem okTheLadyOfOtariaLine : StaticSpec.check (DRS.ofList []) theLadyOfOtariaLine = [] := by decide

/-- Clergy of the Holy Nimbus -/
def clergyOfTheHolyNimbus : Ability :=
  static (replace (would any (destroy thisCreature)) (instead (regeneration it)))
theorem okClergyOfTheHolyNimbus : Ability.check (DRS.ofList []) clergyOfTheHolyNimbus = [] := by decide
/-- Rampant Frogantua -/
def rampantFrogantuaPump : Ability :=
  static (getsPt thisCreature
    (up (forEach 10 (and [isPlayer,
      did which loseGame thisGame])))
    (up (forEach 10 (and [isPlayer,
      did which loseGame thisGame]))))
theorem okRampantFrogantuaPump : Ability.check (DRS.ofList []) rampantFrogantuaPump = [] := by decide
/-- Maskwood Nexus -/
def maskwoodNexusTypes : Ability :=
  static (qualityChange (and [
              allOf (and [isCreature, youControl]),
              allOf (and [isCreature, isSpell, youControl]),
              allOf (and [isCard, isCreature, youOwn, not (in_ battlefield)]) ]) .adds (everyTypeOf
      .creature))
theorem okMaskwoodNexusTypes : Ability.check (DRS.ofList []) maskwoodNexusTypes = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorEquippedPermanent : Ability :=
  static (qualityChange (equipped permanent) .adds
    (bundle { characteristics := { types := [.creature] } }))
theorem okLuxiorEquippedPermanent : Ability.check (DRS.ofList []) luxiorEquippedPermanent = [] := by decide
/-- Nahiri, the Unforgiving's compleated reminder -/
def nahiriCompleatedEntry : Ability :=
  static (entersWithFewerCounters thisPlaneswalker 2 loyaltyCounter)
theorem okNahiriCompleatedEntry : Ability.check (DRS.ofList []) nahiriCompleatedEntry = [] := by decide
/-- Nimble Mongoose -/
def nimbleMongoose : Ability :=
  threshold
    (static (onlyWhile (getsPt thisCreature (up 2) (up 2))
      (compareAmt (countOf (cardIn yourGraveyard)) .atLeast 7)))
theorem okNimbleMongoose : Ability.check (DRS.ofList []) nimbleMongoose = [] := by decide
/-- Anax, Hardened in the Forge -/
def anaxPowerDefinition : Ability :=
  static (ptDefinition thisCreature .powerAlone (devotion you (color .red)))
theorem okAnaxPowerDefinition : Ability.check (DRS.ofList []) anaxPowerDefinition = [] := by decide
/-- Aspect of Wolf -/
def aspectOfWolf : Ability :=
  static (conjunction [ getsPt (enchanted creature) (up (letter .x)) (up (letter .y)),
      letterDefinition .x (half .down
        (countOf (and [forest, youControl]))),
      letterDefinition .y (half .up
        (countOf (and [forest, youControl]))) ])
theorem okAspectOfWolf : Ability.check (DRS.ofList []) aspectOfWolf = [] := by decide
/-- Lhurgoyf -/
def lhurgoyfDefinition : Ability :=
  static (ptDefinitions thisCreature (countOf (and [isCreature, in_ graveyard]))
    (plus thatMuch 1))
theorem okLhurgoyfDefinition : Ability.check (DRS.ofList []) lhurgoyfDefinition = [] := by decide

/-- Deflecting Swat -/
def deflectingSwatCommanderAltCost : Ability :=
  static (onlyWhile (altCost this_)
    (exists_ (and [hasDesignation commander, youControl])))
theorem okDeflectingSwatCommanderAltCost :
    Ability.check (DRS.ofList []) deflectingSwatCommanderAltCost = [] := by decide

/-- Voltage Surge's declaration -/
def voltageSurgeAddedCost : Ability :=
  static (addedCost (sacrifice (a isArtifact)) true)
theorem okVoltageSurgeAddedCost : Ability.check (DRS.ofList []) voltageSurgeAddedCost = [] := by decide
/-- Tarmogoyf -/
def tarmogoyfDefinition : Ability :=
  static (ptDefinitions thisCreature
    (distinctCount .cardType (allOf (cardIn (graveyardOf (bare isPlayer)))))
    (plus thatMuch 1))
theorem okTarmogoyfDefinition : Ability.check (DRS.ofList []) tarmogoyfDefinition = [] := by decide
/-- Consuming Blob's definition -/
def consumingBlobDefinition : Ability :=
  static (ptDefinitions thisCreature (distinctCount .cardType (allOf (cardIn yourGraveyard)))
    (plus thatMuch 1))
theorem okConsumingBlobDefinition : Ability.check (DRS.ofList []) consumingBlobDefinition = [] := by decide
/-- Nighthawk Scavenger's definition -/
def nighthawkScavengerDefinition : Ability :=
  static (ptDefinition thisCreature .powerAlone
    (plus 1
      (distinctCount .cardType (allOf (cardIn (graveyardOf yourOpponents))))))
theorem okNighthawkScavengerDefinition : Ability.check (DRS.ofList []) nighthawkScavengerDefinition = [] := by
  decide
/-- Faeburrow Elder's pump -/
def faeburrowElderPump : Ability :=
  static (getsPt thisCreature
    (up (times 1
      (distinctCount .color (allOf (and [isPermanent, youControl])))))
    (up (times 1
      (distinctCount .color (allOf (and [isPermanent, youControl]))))))
theorem okFaeburrowElderPump : Ability.check (DRS.ofList []) faeburrowElderPump = [] := by decide
/-- Bonds of Faith -/
def bondsOfFaithPump : Ability :=
  static (onlyWhile
    (getsPt (enchanted creature) (up 2) (up 2))
    (is it human))
theorem okBondsOfFaithPump : Ability.check (DRS.ofList []) bondsOfFaithPump = [] := by decide

/-- Lantern of Insight -/
def lanternOfInsightRider : Ability :=
  static (visibility .reveal (bare isPlayer) topOfLibrary)
theorem okLanternOfInsightRider : Ability.check (DRS.ofList []) lanternOfInsightRider = [] := by decide

/-- Phyrexian Unlife -/
def phyrexianUnlifeImmunity : Ability :=
  (match (cardNamed "Phyrexian Unlife").card with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons part _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okPhyrexianUnlifeImmunity : Ability.check (DRS.ofList []) phyrexianUnlifeImmunity = [] := by decide
/-- Umbris, Fear Manifest -/
def umbrisPump : Ability :=
  static (getsPt thisCreature
    (up (forEach 1
      (and [isCard, opponentsOwn, in_ exileZone])))
    (up (forEach 1
      (and [isCard, opponentsOwn, in_ exileZone]))))
theorem okUmbrisPump : Ability.check (DRS.ofList []) umbrisPump = [] := by decide
/-- Turbulent Fen -/
def turbulentFen : Ability :=
  static (entersTappedUnless thisLand
    (compareAmt (countOf (and [isLand, opponentsControl]))
      .atLeast 8))
theorem okTurbulentFen : Ability.check (DRS.ofList []) turbulentFen = [] := by decide
/-- Bastion Protector -/
def bastionProtectorPump : Ability :=
  static (getsPt
    (allOf (and [isCreature, hasDesignation commander, youControl]))
    (up 2) (up 2))
theorem okBastionProtectorPump : Ability.check (DRS.ofList []) bastionProtectorPump = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorTypeSetting : Ability :=
  static (conjunction [ qualityChange (equipped permanent) .loses
        (bundle { characteristics := { types := [.planeswalker] } }),
      qualityChange it .adds (bundle { characteristics := { types := [.creature] } }) ])
theorem okLuxiorTypeSetting : Ability.check (DRS.ofList []) luxiorTypeSetting = [] := by decide
/-- Kasmina, Enigma Sage -/
def kasminaLoyaltySharing : StaticSpec :=
  abilityGrantFrom (allOf (and [isPlaneswalker, youControl, isOther])) this_ [.loyalty]
theorem okKasminaLoyaltySharing : StaticSpec.check (DRS.ofList []) kasminaLoyaltySharing = [] := by decide
/-- Nicol Bolas, Dragon-God -/
def nicolBolasDragonGodSharing : StaticSpec :=
  abilityGrantFrom this_ (allOf
                             (and [isPlaneswalker, isOther, in_ battlefield])) [.loyalty]
theorem okNicolBolasDragonGodSharing : StaticSpec.check (DRS.ofList []) nicolBolasDragonGodSharing = [] := by
  decide
/-- Myr Welder -/
def myrWelderBorrowedAbilities : StaticSpec :=
  abilityGrantFrom thisCreature (allOf (in_ (exiledWith this_))) [.anyActivated]
theorem okMyrWelderBorrowedAbilities : StaticSpec.check (DRS.ofList []) myrWelderBorrowedAbilities = [] := by
  decide
/-- Sharkey, Tyrant of the Shire -/
def sharkeyBorrowedLandAbilities : StaticSpec :=
  abilityGrantFrom this_ (allOf (and [isLand, opponentsControl])) [.anyActivated]
                                                                                      isManaAbility
theorem okSharkeyBorrowedLandAbilities : StaticSpec.check (DRS.ofList []) sharkeyBorrowedLandAbilities = [] := by
  decide

/-- Emissary of Grudges -/
def emissaryOfGrudgesEntry : StaticSpec :=
  entersChoosingPlayerSecretly thisCreature anOpponent
theorem okEmissaryOfGrudgesEntry : StaticSpec.check (DRS.ofList []) emissaryOfGrudgesEntry = [] := by decide

def aerialAssault : Action := destroy (target (and [isCreature, isTapped]))
theorem okAerialAssault : Action.check (DRS.ofList []) aerialAssault = [] := by decide
def asphyxiate : Action := destroy (target (and [isCreature, isUntapped]))
theorem okAsphyxiate : Action.check (DRS.ofList []) asphyxiate = [] := by decide
def vindicate : Action := destroy (target isPermanent)
theorem okVindicate : Action.check (DRS.ofList []) vindicate = [] := by decide

/-- Gray Merchant of Asphodel -/
def grayMerchantDrain : Action :=
  sequentially
    [instruct eachOpponent (loseLife (letter .x)), define .x (devotion you (color .black))]
theorem okGrayMerchantDrain : Action.check (DRS.ofList []) grayMerchantDrain = [] := by decide

end Semantics.Cards
