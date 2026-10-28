import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Mana

Port of `idris/src/Experimental/Cards/Mana.idr`: the printed cards of the Mana family and the
bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def boshIronGolem : Ability :=
  activated (compound [[generic 3, pip .red], sacrifice (a isArtifact)])
    (instruct this_
       (dealDamage
          (statOf (.stat .manaValue) (the (and [was (sacrifice which), isArtifact])))
        (target isAnyTarget)))
theorem okBoshIronGolem : Ability.check (DRS.ofList []) boshIronGolem = [] := by decide
def pyromancy : Ability :=
  activated (compound [[generic 3], discard (aAtRandom (cardIn yourHand))])
    (instruct thisEnchantment
      (dealDamage (statOf (.stat .manaValue) (the (and [was (discard which), isCard])))
        (target isAnyTarget)))
theorem okPyromancy : Ability.check (DRS.ofList []) pyromancy = [] := by decide
def luckyOffering : Action :=
  sequentially
    [ destroy (target (and [isArtifact, hasManaValue .atMost 3])),
      gainLife 3 ]
theorem okLuckyOffering : Action.check (DRS.ofList []) luckyOffering = [] := by decide
def overload : Action :=
  doOnlyIf (destroy (target isArtifact)) (compareAmt (statOf (.stat .manaValue) it) .atMost 2)
theorem okOverload : Action.check (DRS.ofList []) overload = [] := by decide
def austereCommand : Action :=
  chooseModes (exactly 2)
    [ destroy (allOf isArtifact),
      destroy (allOf isEnchantment),
      destroy (allOf (and [isCreature, hasManaValue .atMost 3])),
      destroy (allOf (and [isCreature, hasManaValue .atLeast 4])) ]
theorem okAustereCommand : Action.check (DRS.ofList []) austereCommand = [] := by decide

def boommobile : Ability :=
  when (does thisArtifact enter)
    (addMana 4 (ofAnyColor .sameColor) [spendOnly [toActivate none]])
theorem okBoommobile : Ability.check (DRS.ofList []) boommobile = [] := by decide

/-- Qarsi Deceiver: "Spend this mana only to cast a face-down creature spell, pay a mana cost to
turn a manifested creature face up, or pay a morph cost." The turn-face-up purpose does not yet
say "a mana cost", so it also admits a manifested card's disguise cost [CR#701.40d], which the text
forbids: `planned/lean-v3-turn-face-up-spend-cost.md`. -/
def qarsiDeceiverMorphSpend : Ability :=
  activated tapSymbol
    (addMana 1 (manaOf [[.colorless]])
      [spendOnly
        [ toCast (and [isCreature, isFaceDown]),
          toTurnFaceUp (and [isCreature, was (manifest which) thisObject]),
          toPay (.ofKeyword morph) ]])
theorem okQarsiDeceiverMorphSpend : Ability.check (DRS.ofList []) qarsiDeceiverMorphSpend = [] := by decide

/-- Rootcoil Creeper -/
def rootcoilCreeperGraveyardMana : Ability :=
  activated tapSymbol
    (addMana 2 (ofAnyColor .sameColor)
      [spendOnly [toCast (and [isSpell,
          did any (cast which (from_ := yourGraveyard)) thisObject])]])
theorem okRootcoilCreeperGraveyardMana : Ability.check (DRS.ofList []) rootcoilCreeperGraveyardMana = [] := by
  decide

/-- Steelswarm Operator -/
def steelswarmOperatorMana : Action :=
  addMana 1 (manaOf [[.of .blue, .of .blue]])
    [spendOnly [toActivate (some (and [isSource, isArtifact]))]]
theorem okSteelswarmOperatorMana : Action.check (DRS.ofList []) steelswarmOperatorMana = [] := by decide

/-- Nahiri, the Unforgiving's 0: "… card with mana value less than Nahiri's loyalty from your
graveyard". On the battlefield her loyalty is her loyalty-counter count [CR#306.5c], not her
printed number, and the bound reads it as the ability resolves. -/
def nahiriLoyaltyRead : Predicate :=
  and [ isCreature, in_ yourGraveyard,
         hasManaValue .less (statOf (.stat .loyalty) this_) ]
theorem okNahiriLoyaltyRead : Predicate.checkAt .object (DRS.ofList []) nahiriLoyaltyRead = [] := by decide

/-- Caustic Bronco -/
def causticBroncoLoss : Action :=
  sequentially
    [ reveal (topSlice 1),
      putInto (that card) hand,
      doOnlyIf (loseLife (statOf (.stat .manaValue) it))
        (not (is thisCreature (hasDesignation saddled)))
        (instruct eachOpponent (loseLife thatMuch)) ]
theorem okCausticBroncoLoss : Action.check (DRS.ofList []) causticBroncoLoss = [] := by decide

/-- Dark Fortress -/
def darkFortressMana : Ability :=
  activatedOnlyIf tapSymbol (addMana 1 (manaOf [[.of .black], [.of .red]]))
    (or [ did thisLand enter soFarThisTurn,
           exists_ (and [isLand, hasSupertype .basic, youControl]) ])
theorem okDarkFortressMana : Ability.check (DRS.ofList []) darkFortressMana = [] := by decide

/-- Karametra's Acolyte -/
def karametrasAcolyte : Ability :=
  activated tapSymbol (addMana (devotion you (color .green)) (manaOf [[.of .green]]))
theorem okKarametrasAcolyte : Ability.check (DRS.ofList []) karametrasAcolyte = [] := by decide

/-- Birthing Pod -/
def birthingPodSearch : Ability :=
  activated
    (compound [[generic 1, phyrexianPip .green], tapSymbol, sacrifice (a isCreature)])
    (search yourLibrary
      (a (and [isCreature,
             hasManaValue .eq (plus 1 (statOf (.stat .manaValue)
                   (the (and [was (sacrifice which), isCreature]))))])))
theorem okBirthingPodSearch : Ability.check (DRS.ofList []) birthingPodSearch = [] := by decide

/-- Hibernation's End -/
def hibernationsEndTrigger : Ability :=
  whenever
      (does you (pay anyCost .once .paid (some thisEnchantment) (some cumulativeUpkeep)))
    (may
      (sequentially
        [ search yourLibrary
            (a (and [isCreature, hasManaValue .eq (countersOn ageCounter thisEnchantment)])),
          putOntoBattlefield (that card),
          shuffle ]))
theorem okHibernationsEndTrigger : Ability.check (DRS.ofList []) hibernationsEndTrigger = [] := by decide

/-- Bloom Tender -/
def bloomTenderMana : Ability :=
  vivid
    (activated tapSymbol
      (doForEachKind .color (allOf (and [isPermanent, youControl])) .color
        (addMana 1 ofTheChosenColor)))
theorem okBloomTenderMana : Ability.check (DRS.ofList []) bloomTenderMana = [] := by decide

/-- Tarnation Vista -/
def tarnationVistaMana : Ability :=
  activated (compound [[generic 1], tapSymbol])
    (doForEachKind .color (allOf (and [isPermanent, isMonocolored, youControl])) .color
      (addMana 1 ofTheChosenColor))
theorem okTarnationVistaMana : Ability.check (DRS.ofList []) tarnationVistaMana = [] := by decide

/-- Soul Shatter -/
def soulShatter : Action :=
  instruct eachOpponent (sacrifice
    (a (and [or [isCreature, isPlaneswalker],
              superlative .max (.stat .manaValue)
                (and [or [isCreature, isPlaneswalker], controlledBy they])])))
theorem okSoulShatter : Action.check (DRS.ofList []) soulShatter = [] := by decide

/-- Padeem, Consul of Innovation -/
def padeemConsulOfInnovation : Ability :=
  at_ (beginningOfYour .upkeep)
    (if_ := exists_ (and [isArtifact, youControl,
      superlative .max (.stat .manaValue) (and [isArtifact, in_ battlefield])]))
    (draw 1)
theorem okPadeemConsulOfInnovation : Ability.check (DRS.ofList []) padeemConsulOfInnovation = [] := by decide

/-- Delighted Halfling -/
def delightedHalflingMana : Ability :=
  activated tapSymbol
    (addMana 1 (ofAnyColor .sameColor)
      [ whenSpentOn .affectsIt true (a (and [hasSupertype .legendary, isSpell]))
          (establish (cant any (counter (that .spell)))) ])
theorem okDelightedHalflingMana : Ability.check (DRS.ofList []) delightedHalflingMana = [] := by decide

/-- Thran Turbine -/
def thranTurbineMana : Action :=
  addMana 1 (manaOf [[.colorless, .colorless]]) [cantSpendOn [toCast isSpell]]
theorem okThranTurbineMana : Action.check (DRS.ofList []) thranTurbineMana = [] := by decide

/-- Su-Chi Cave Guard -/
def suChiCaveGuardDies : Ability :=
  when (does thisCreature die)
    (sequentially
      [ addMana 1
          (manaOf [[.colorless, .colorless, .colorless, .colorless,
                   .colorless, .colorless, .colorless, .colorless]]),
        establishThroughout (manaRetention you thisMana) untilEndOfTurn ])
theorem okSuChiCaveGuardDies : Ability.check (DRS.ofList []) suChiCaveGuardDies = [] := by decide

/-- Omnath, Locus of Mana -/
def omnathLocusOfManaPersistence : StaticSpec := manaRetention you (unspent (some (.of .green)))
theorem okOmnathLocusOfManaPersistence :
    StaticSpec.check (DRS.ofList []) omnathLocusOfManaPersistence = [] := by decide

/-- Ice Cauldron -/
def iceCauldronNotedMana : Ability :=
  activated
    (compound [tapSymbol,
      (removeCounters (exactly 1) chargeCounter thisArtifact)])
    (addMana 1 (ofTheLastNotedType thisArtifact)
      [spendOnly [toCast (cardIn (exiledWith thisArtifact))]])
theorem okIceCauldronNotedMana : Ability.check (DRS.ofList []) iceCauldronNotedMana = [] := by decide

/-- Firemind Vessel -/
def firemindVesselMana : Ability :=
  activated tapSymbol (addMana 2 (ofAnyColor .distinctColors))
theorem okFiremindVesselMana : Ability.check (DRS.ofList []) firemindVesselMana = [] := by decide

/-- Goblin Clearcutter -/
def goblinClearcutterMana : Ability :=
  activated
    (compound [tapSymbol, sacrifice (a forest)])
    (addMana 3 (ofAnyColorAmong [.red, .green]))
theorem okGoblinClearcutterMana : Ability.check (DRS.ofList []) goblinClearcutterMana = [] := by decide

/-- Psychic Vortex -/
def psychicVortexUpkeep : Ability := cumulativeUpkeep (draw 1)
theorem okPsychicVortexUpkeep : Ability.check (DRS.ofList []) psychicVortexUpkeep = [] := by decide

/-- Varchild's War-Riders -/
def varchildsWarRidersUpkeep : Ability :=
  cumulativeUpkeep
    (instruct anOpponent (create 1 (creatureToken 1 1 [.red] [survivor])))
theorem okVarchildsWarRidersUpkeep : Ability.check (DRS.ofList []) varchildsWarRidersUpkeep = [] := by decide

end Semantics.Cards
