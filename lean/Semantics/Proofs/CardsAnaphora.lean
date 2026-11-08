import Semantics.Cards.Data.Fixtures
import Semantics.Check
import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsHelpers

-- Canon cards are `noncomputable` data (`Semantics.Cards.Data`); so is what reads them.
noncomputable section


/-!
# Semantics.Cards.Anaphora

Port of `idris/src/Experimental/Cards/Anaphora.idr`: the printed cards of the Anaphora family
and the phrase-level bench items beside them, each paired with an `ok…` theorem that runs the
checker on it at the bindings its Idris type named; the file's own `Refl` pins are theorems.

Not ported: `ichorElixirPlanarDice` (a planar die is Planechase, out of scope).
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (a each counted the)
open Semantics.Templates.Predicate (and or was)
open Semantics.Templates.ManaSymbol (generic)
open Semantics.Templates.Location (and or)
open Semantics.Templates.NounPhrase (and or)
open Semantics.Templates.StaticSpec (replace)

open Semantics.Definitions.CardType (creature enchantment)
open Semantics.Templates.Ability (spell)
open Semantics.Templates.Action (replace target)
open Semantics.Templates.Condition (and or was)
open Semantics.Templates.CostShift (generic)
open Semantics.Templates.DetPhrase (a each the)
open Semantics.Templates.NounWord (creature enchantment)
open Semantics.Templates.SliceCount (counted)

namespace Semantics.Cards

def cloudshift : Action :=
  sequentially
    [ Semantics.Definitions.Action.exile (target (and [isCreature, youControl])),
      returnToBattlefield (that card) [(under you)] ]
theorem okCloudshift : Action.check (DRS.ofList []) cloudshift = [] := by decide

def bitterDownfall : Action :=
  sequentially [destroy (target isCreature), instruct (controllerOf it) (loseLife 2)]
theorem okBitterDownfall : Action.check (DRS.ofList []) bitterDownfall = [] := by decide

def suspendedSentence : Action :=
  sequentially
    [ destroy (target (and [isCreature, anOpponentControls])),
      instruct (that Semantics.Templates.NounWord.player) (loseLife 3) ]
theorem okSuspendedSentence : Action.check (DRS.ofList []) suspendedSentence = [] := by decide

def flickeringSpirit : Action :=
  sequentially [Semantics.Definitions.Action.exile thisCreature, returnToBattlefield it [(under (ownerOf it))]]
theorem okFlickeringSpirit : Action.check (DRS.ofList []) flickeringSpirit = [] := by decide

/-- Bench item: "Mill a card. Return the card put into your graveyard this way to your hand." One
milled card is read back as the one card moved [CR#701.17a]. -/
def millACardReturnIt : Action :=
  sequentially
    [ mill (.lit 1),
      returnToHand (the (and [isCard, was (putInto which yourGraveyard) thisWay])) ]
theorem okMillACardReturnIt : Action.check (DRS.ofList []) millACardReturnIt = [] := by decide

/-- Bond of Revival -/
def bondOfRevival : Action :=
  sequentially
    [ returnToBattlefield (target (and [isCreature, in_ yourGraveyard]))
        (from_ := yourGraveyard),
      gainHaste it untilYourNextTurn ]
theorem okBondOfRevival : Action.check (DRS.ofList []) bondOfRevival = [] := by decide

def vraskasStoneglare : Action :=
  sequentially [destroy (target isCreature), gainLife (toughnessOf it)]
theorem okVraskasStoneglare : Action.check (DRS.ofList []) vraskasStoneglare = [] := by decide

def phthisis : Action :=
  sequentially
    [ destroy (target isCreature),
      instruct (controllerOf it) (loseLife
        (Semantics.Templates.Amount.plus (powerOf it) (toughnessOf it))) ]
theorem okPhthisis : Action.check (DRS.ofList []) phthisis = [] := by decide

def foulTongueShriek : Action :=
  sequentially
    [ instruct (target isOpponent) (loseLife
        (forEach 1 (and [isAttacking, isCreature, youControl]))),
      gainLife thatMuch ]
theorem okFoulTongueShriek : Action.check (DRS.ofList []) foulTongueShriek = [] := by decide

def impulse : Action :=
  sequentially
    [ Semantics.Templates.Action.lookAt (topSlice 4), putInto (someOf (exactly 1) them) Semantics.Templates.Location.hand,
      putInto theRest onBottom (riders := [(inOrder .anyOrder)]) ]
theorem okImpulse : Action.check (DRS.ofList []) impulse = [] := by decide

def anticipate : Action :=
  sequentially
    [ Semantics.Templates.Action.lookAt (topSlice 3), putInto (someOf (exactly 1) them) Semantics.Templates.Location.hand,
      putInto theRest onBottom (riders := [(inOrder .anyOrder)]) ]
theorem okAnticipate : Action.check (DRS.ofList []) anticipate = [] := by decide

/-- In *Flow State*'s replacement, "them" reads the three cards looked at (referent 1, in the
library), not the instant and sorcery cards the existential published nor the part and remainder
of the replaced action, and they are the one group "the rest" is taken from. -/
theorem flowStateThemAndTheRestReadTheCardsLookedAt :
    let looked := (stepAction 512 .postEvent .obj (DRS.ofList []) (Semantics.Templates.Action.lookAt (topSlice 3))).1
    let replaced := (stepAction 512 .preEvent .obj looked (sequentially
      [ putInto (someOf (exactly 1) them) Semantics.Templates.Location.hand,
        putInto theRest onBottom (riders := [(inOrder .anyOrder)]) ])).1
    let instantAndSorcery := thereAre (both (a (and [isInstant, cardIn yourGraveyard]))
      (a (and [isSorcery, cardIn yourGraveyard])))
    let start := nextClause (replacementStart looked replaced)
    let after :=
      nextClause (markReplacementDomain start (stepCondition 512 start instantAndSorcery).1)
    (resolveCandidates .bare .many after).map (fun ref => (ref.id, ref.zone)) =
        [(1, some .library)] ∧
      countGroups .object after = 1 ∧ countedGroupSize .object after = some 3 := by
  decide

/-- "Look at the top three cards of your library. Put one of them into your hand and the rest on
the bottom of your library in any order. Instead, put two of them into your hand and the rest on
the bottom of your library in any order": the replacement reads what the replaced action read
[CR#614.15]. It is not refused `theRestFits`, and its "them" reads the cards looked at (referent
1), not the replaced action's remainder. -/
theorem selfReplacementReadsWhatTheReplacedActionRead :
    let putOne := sequentially
      [ putInto (someOf (exactly 1) them) Semantics.Templates.Location.hand,
        putInto theRest onBottom (riders := [(inOrder .anyOrder)]) ]
    let putTwo := sequentially
      [ putInto (someOf (exactly 2) them) Semantics.Templates.Location.hand,
        putInto theRest onBottom (riders := [(inOrder .anyOrder)]) ]
    let looked := (stepAction 512 .postEvent .obj (DRS.ofList []) (Semantics.Templates.Action.lookAt (topSlice 3))).1
    let replaced := (stepAction 512 .preEvent .obj looked putOne).1
    let start := nextClause (replacementStart looked replaced)
    Action.check (DRS.ofList [])
        (sequentially [Semantics.Templates.Action.lookAt (topSlice 3), replace putOne (instead putTwo)]) = [] ∧
      (resolveCandidates .bare .many start).map (·.id) = [1] ∧
      countGroups .object start = 1 ∧ countParts .object start = 0 := by
  decide

def revealFourPartition : Action :=
  sequentially
    [ Semantics.Definitions.Action.reveal (topSlice 4), putInto (someOf (exactly 1) (those card)) Semantics.Templates.Location.hand,
      putInto theRest Semantics.Templates.Location.graveyard ]
theorem okRevealFourPartition : Action.check (DRS.ofList []) revealFourPartition = [] := by decide

def exileFourOfThem : Action :=
  sequentially
    [ Semantics.Templates.Action.lookAt (topSlice 8), Semantics.Definitions.Action.exile (someOf (exactly 4) them),
      putInto theRest onTop (riders := [(inOrder .anyOrder)]) ]
theorem okExileFourOfThem : Action.check (DRS.ofList []) exileFourOfThem = [] := by decide

def sylvanScrying : Action :=
  sequentially [search yourLibrary (a isLand), Semantics.Definitions.Action.reveal it, putInto it Semantics.Templates.Location.hand, shuffle]
theorem okSylvanScrying : Action.check (DRS.ofList []) sylvanScrying = [] := by decide

def searchToBattlefield : Action :=
  sequentially [search yourLibrary (a isCreature), putOntoBattlefield it, shuffle]
theorem okSearchToBattlefield : Action.check (DRS.ofList []) searchToBattlefield = [] := by decide

def glimpseTheUnthinkable : Action := instruct (target isPlayer) (mill 10)
theorem okGlimpseTheUnthinkable : Action.check (DRS.ofList []) glimpseTheUnthinkable = [] := by decide

def millThenReadGroup : Action :=
  sequentially [mill 3, Semantics.Definitions.Action.exile (those card)]
theorem okMillThenReadGroup : Action.check (DRS.ofList []) millThenReadGroup = [] := by decide

def takeIntoCustody : Action :=
  sequentially [tap (target isCreature), skipUntap it 1]
theorem okTakeIntoCustody : Action.check (DRS.ofList []) takeIntoCustody = [] := by decide

/-- Frenzied Gorespawn -/
def frenziedGorespawnGoad : Action :=
  doForEach eachOpponent
    (gainDesignation
      (target (and [isCreature, controlledBy (that Semantics.Templates.NounWord.player)])) goaded)
theorem okFrenziedGorespawnGoad : Action.check (DRS.ofList []) frenziedGorespawnGoad = [] := by decide

/-- Consecrate // Consume -/
def consume : Ability :=
  spell (sequentially
    [ instruct (target isPlayer) (sacrifice
        (a (and [ isCreature,
                   superlative .max (.stat .power)
                     (and [isCreature, controlledBy they]) ]))),
      gainLife (powerOf it) ])
theorem okConsume : Ability.check (DRS.ofList []) consume = [] := by decide

def riseFromTheGrave : Action :=
  sequentially
    [ putOntoBattlefieldUnderYourControl (target (and [isCreature, in_ Semantics.Templates.Location.graveyard]))
        (from_ := Semantics.Templates.Location.graveyard),
      become (that creature)
        { characteristics := { colors := [.black], subtypes := [zombie] } } ]
theorem okRiseFromTheGrave : Action.check (DRS.ofList []) riseFromTheGrave = [] := by decide

def everAfter : Action :=
  sequentially
    [ returnToBattlefield (targets (upTo 2) (and [isCreature, in_ yourGraveyard]))
        (from_ := yourGraveyard),
      become (those creature)
        { characteristics := { colors := [.black], subtypes := [zombie] } },
      putInto this_ onBottom ]
theorem okEverAfter : Action.check (DRS.ofList []) everAfter = [] := by decide

def martyrsCry : Action :=
  sequentially
    [ Semantics.Definitions.Action.exile (allOf (and [isCreature, hasColor .white])),
      doForEach (each (and [isCreature, was (Semantics.Definitions.Action.exile which) thisWay]))
        (instruct (controllerOf it) (draw 1)) ]
theorem okMartyrsCry : Action.check (DRS.ofList []) martyrsCry = [] := by decide

def anotherRound : Action :=
  sequentially
    [ Semantics.Definitions.Action.exile (counted anyNumber (and [isCreature, youControl])),
      returnToBattlefield them [(under (ownerOf them))],
      repeat_ (moreTimes (letter .x)) ]
theorem okAnotherRound : Action.check (DRS.ofList []) anotherRound = [] := by decide

/-- Eradicate -/
def eradicateSearch : Action :=
  sequentially
    [ Semantics.Definitions.Action.exile (target (and [isCreature, non (hasColor .black)])),
      searchZonesOf (controllerOf (that card)) (allOf (shares .cardName .share (that card))),
      Semantics.Definitions.Action.exile them,
      instruct (that Semantics.Templates.NounWord.player) shuffle ]
theorem okEradicateSearch : Action.check (DRS.ofList []) eradicateSearch = [] := by decide

/-- Deem Inferior -/
def deemInferior : Action :=
  instruct (ownerOf (target (and [isPermanent, non isLand])))
      (put it (or [(fromTop 2 Semantics.Templates.Location.library), (fromBottom 2 Semantics.Templates.Location.library)]))
theorem okDeemInferior : Action.check (DRS.ofList []) deemInferior = [] := by decide

/-- Write into Being -/
def writeIntoBeingPlacement : Action :=
  sequentially
    [ Semantics.Templates.Action.lookAt (topSlice 2),
      manifestPlacement (someOf (exactly 1) them),
      putInto theOther (or [onTop, onBottom]) ]
theorem okWriteIntoBeingPlacement : Action.check (DRS.ofList []) writeIntoBeingPlacement = [] := by decide

/-- Mystical Tutor -/
def mysticalTutor : Action :=
  sequentially
    [search yourLibrary (a isInstantOrSorcery), Semantics.Definitions.Action.reveal it, shuffle, putInto it onTop]
theorem okMysticalTutor : Action.check (DRS.ofList []) mysticalTutor = [] := by decide

/-- Demonic Tutor -/
def demonicTutor : Action :=
  sequentially [search yourLibrary (a isCard), putInto it Semantics.Templates.Location.hand, shuffle]
theorem okDemonicTutor : Action.check (DRS.ofList []) demonicTutor = [] := by decide

/-- Thalia's Lancers -/
def thaliasLancersSearch : Action :=
  may
    (sequentially
      [ search yourLibrary (a (and [hasSupertype .legendary])), Semantics.Definitions.Action.reveal it,
        putInto it Semantics.Templates.Location.hand, shuffle ])
theorem okThaliasLancersSearch : Action.check (DRS.ofList []) thaliasLancersSearch = [] := by decide

def contrabandLivestock : Action :=
  sequentially
    [ Semantics.Definitions.Action.exile (target isCreature),
      rollDice 1 (.sides 20),
      applyResultsTable
        [ rollRow (oneThrough 9)
            (instruct (controllerOf it) (create 1 (Semantics.TokenSpec.written ((creatureToken 4 4 [.green] [ox]))))),
          rollRow (fromTo 10 19)
            (instruct (controllerOf it) (create 1 (Semantics.TokenSpec.written ((creatureToken 2 2 [.green] [boar]))))),
          rollRow (exactly 20)
            (instruct (controllerOf it) (create 1 (Semantics.TokenSpec.written ((creatureToken 0 1 [.white] [goat]))))) ] ]
theorem okContrabandLivestock : Action.check (DRS.ofList []) contrabandLivestock = [] := by decide

/-- Hypnotic Specter -/
def hypnoticSpecterDiscard : Action := instruct they (discard (aAtRandom (cardIn Semantics.Templates.Location.hand)))
theorem okHypnoticSpecterDiscard :
    Action.check (DRS.ofList [{ det := .the, plur := .one, payload := .player Bool.false }]) hypnoticSpecterDiscard = [] := by decide

/-- Wyll, Blade of Frontiers -/
def wyllExtraDie : Action :=
  replaceEvent (would (Semantics.Subject.actor (you)) (rollDice anyAmount .anyDie))
    (instead (sequentially
      [ rollDice (Semantics.Templates.Amount.plus thatMuch 1) .thoseDice,
        ignoreOutcomes (extreme .lowest) ]))
    none
theorem okWyllExtraDie : Action.check (DRS.ofList []) wyllExtraDie = [] := by decide

/-- Investigator's Journal's count -/
def greatestCreaturesAPlayerControls : Amount :=
  (match (Semantics.Cards.«Investigator's Journal») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.static (StaticSpec.replace _ (Replacement.withRiders (List.cons
        (MoveRider.withCounters part _ _) _)) _ _ _)) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okGreatestCreaturesAPlayerControls :
    Amount.check (DRS.ofList []) greatestCreaturesAPlayerControls = [] := by decide
/-- Cavern-Hoard Dragon -/
def greatestArtifactsAnOpponentControls : Amount :=
  aggregateOver .max isOpponent (countOf (and [isArtifact, controlledBy they]))
theorem okGreatestArtifactsAnOpponentControls :
    Amount.check (DRS.ofList []) greatestArtifactsAnOpponentControls = [] := by decide

def anyTargetAnnounced : NounPhrase := target isAnyTarget
theorem okAnyTargetAnnounced : NounPhrase.checkAt none (DRS.ofList []) anyTargetAnnounced = [] := by decide
def playerOrPlaneswalkerAnnounced : NounPhrase := targetPlayerOrPlaneswalker
/-- Chain of Plasma -/
def chainOfPlasmaOfferee : NounPhrase := splitOverPermanent
theorem okChainOfPlasmaOfferee :
    NounPhrase.checkAt (some .player)
        ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true anyTargetAnnounced).1) chainOfPlasmaOfferee =
        [] := by
  decide
/-- Chain Lightning -/
def chainLightningPayer : NounPhrase := splitOverPermanent
theorem okChainLightningPayer :
    NounPhrase.checkAt (some .player)
        ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true anyTargetAnnounced).1) chainLightningPayer =
        [] := by
  decide
/-- Flames of the Blood Hand -/
def flamesOfTheBloodHandSubject : NounPhrase := splitOverPlaneswalker
theorem okFlamesOfTheBloodHandSubject :
    NounPhrase.checkAt (some .player)
        ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true playerOrPlaneswalkerAnnounced).1)
        flamesOfTheBloodHandSubject = [] := by
  decide
/-- Flaming Gambit -/
def flamingGambitOfferee : NounPhrase := splitOverPlaneswalker
theorem okFlamingGambitOfferee :
    NounPhrase.checkAt (some .player)
        ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true playerOrPlaneswalkerAnnounced).1)
        flamingGambitOfferee = [] := by
  decide
theorem okEachCreatureThatSplitControls :
    NounPhrase.checkAt (some .object)
        ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true playerOrPlaneswalkerAnnounced).1)
        eachCreatureThatSplitControls = [] := by
  decide

/-- Blessed Respite -/
def blessedRespiteShuffle : Action :=
  instruct (target isPlayer) (shuffleInto (allOf (cardIn (graveyardOf they))))
theorem okBlessedRespiteShuffle : Action.check (DRS.ofList []) blessedRespiteShuffle = [] := by decide

/-- Nekrataal -/
def nekrataalEntry : GameEvent := (match (Semantics.Cards.«Nekrataal») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.triggered part _ _ _ _ _ _ _) _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okNekrataalEntry : GameEvent.check (DRS.ofList []) nekrataalEntry = [] := by decide
def nekrataalDestroy : Action :=
  (match (Semantics.Cards.«Nekrataal») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons
        part _))) _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okNekrataalDestroy :
    Action.check ((stepGameEvent 512 (DRS.ofList []) nekrataalEntry).1) nekrataalDestroy = [] := by decide
def nekrataalRider : DRS :=
  (stepAction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) nekrataalEntry).1) nekrataalDestroy).1
/-- After the destroy, no creature word reaches a creature on the battlefield: the destroyed
creature is in a graveyard, and the Source, "this creature", is a self-referent no noun word
reads (`Determiner.self`; owner ruling 2026-10-24), which this pin used to count. -/
theorem nekrataalNoCreatureWord : countReach (.word creature) .one nekrataalRider = 0 := by
  decide
theorem nekrataalOneDestroyed :
    (participialCandidates (.deed (.action "destroy")) (some .one) none Bool.false nekrataalRider).length = 1 := by
  decide

def sequencedRiderBody : Action :=
  sequentially [Semantics.Definitions.Action.exile (target isArtifact), destroy (target isCreature)]
theorem okSequencedRiderBody : Action.check (DRS.ofList []) sequencedRiderBody = [] := by decide
def sequencedRider : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) sequencedRiderBody).1
theorem sequencedRiderOneDestroyed :
    (participialCandidates (.deed (.action "destroy")) (some .one) none Bool.false sequencedRider).length = 1 := by decide
theorem sequencedRiderOneExiled :
    (participialCandidates (.deed (.action "exile")) (some .one) none Bool.false sequencedRider).length = 1 := by
  decide

/-- Engulfing Flames -/
def engulfingFlamesBody : Action := instruct this_ (dealDamage 1 (target isCreature))
theorem okEngulfingFlamesBody : Action.check (DRS.ofList []) engulfingFlamesBody = [] := by decide
def engulfingFlamesRider : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) engulfingFlamesBody).1
theorem engulfingFlamesNoDestroyStamp :
    (participialCandidates (.deed (.action "destroy")) (some .one) none Bool.false engulfingFlamesRider).length = 0 := by decide
theorem engulfingFlamesBareReadStands : countReach .bare .one engulfingFlamesRider = 1 := by decide

def bioplasmExiledPronoun : NounPhrase := (match (Semantics.Cards.«Bioplasm») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.triggered _ _ _ _ _ _ _ (Action.sequentially (List.cons _ (List.cons
        (Action.doIf (Condition.matches part _) _ _ _) _)))) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okBioplasmExiledPronoun :
    NounPhrase.checkAt (some .object) bioplasmAfterExile bioplasmExiledPronoun = [] := by decide
theorem bioplasmNoTypedRead :
    (participialCandidates (.deed (.action "exile")) (some .one) creature Bool.false bioplasmAfterExile).length
      = 0 := by
  decide
theorem bioplasmExiledCardHasNoType :
    (participialCandidates (.deed (.action "exile")) (some .one) none Bool.false bioplasmAfterExile).map (·.ty)
      = [[]] := by decide
theorem bioplasmTypeTestPreservesExiledIdentity :
    let before := participialCandidates (.deed (.action "exile")) (some .one) none Bool.false bioplasmAfterExile
    let after := participialCandidates (.deed (.action "exile")) (some .one) none Bool.false bioplasmAfterTest
    before.length = 1 ∧ after.map (·.id) = before.map (·.id) := by decide
theorem bioplasmTestRemarksType :
    (participialCandidates (.deed (.action "exile")) (some .one) none Bool.false bioplasmAfterTest).map (·.ty)
      = [[.creature]] := by decide
/-- The narrowed bare read finds the exiled card, whose card carrier remains available. -/
theorem bioplasmTestKeepsCardSlot : countReach (.atSlot .card) .one bioplasmAfterTest = 1 := by
  decide
theorem bioplasmTypedReadStillRefused :
    (participialCandidates (.deed (.action "exile")) (some .one) creature Bool.false bioplasmAfterTest).length
      = 0 := by
  decide
theorem bioplasmTypedCardReadWrites :
    (participialCandidates (.deed (.action "exile")) (some .one) (some (.ofType .card .creature)) Bool.false
      bioplasmAfterTest).length = 1 := by
  decide

/-- Scapeshift -/
def scapeshiftSacrifice : Action := (match (Semantics.Cards.«Scapeshift») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ (Action.sequentially (List.cons part _))) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okScapeshiftSacrifice : Action.check (DRS.ofList []) scapeshiftSacrifice = [] := by decide
def scapeshiftSacrificed : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) scapeshiftSacrifice).1
def scapeshiftSearch : Action := (match (Semantics.Cards.«Scapeshift») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ (Action.sequentially (List.cons _ (List.cons part _)))) _) =>
        some part
      | _ => none
    | _ => none).get (by decide)
theorem okScapeshiftSearch : Action.check scapeshiftSacrificed scapeshiftSearch = [] := by
  decide
def scapeshiftAfterSearch : DRS := (stepAction 512 .postEvent .obj scapeshiftSacrificed scapeshiftSearch).1
theorem scapeshiftTwoGroups : countReach .bare .many scapeshiftAfterSearch = 2 := by decide
theorem scapeshiftOneSearchedGroup :
    (participialCandidates (.deed search.deed) (some .many) none Bool.false scapeshiftAfterSearch).length = 1 := by decide
theorem scapeshiftOneSacrificedGroup :
    (participialCandidates (.deed sacrifice.deed) (some .many) none Bool.false scapeshiftAfterSearch).length = 1 := by decide

/-- Bind to Life, Vastlands Scavenger's adventure -/
def bindToLife : Action :=
  sequentially
    [mill 7, putOntoBattlefield (fromAmong (exactly 1) isCreature them)]
theorem okBindToLife : Action.check (DRS.ofList []) bindToLife = [] := by decide

/-- Glamdring, Foe-hammer's Gleam of Death -/
def gleamOfDeath : Action :=
  sequentially
    [mill 6, putInto (allFromAmong isInstantOrSorcery them) Semantics.Templates.Location.hand]
theorem okGleamOfDeath : Action.check (DRS.ofList []) gleamOfDeath = [] := by decide

/-- Tezzeret, Master of the Bridge -/
def tezzeretAllArtifacts : Action :=
  sequentially
    [ Semantics.Definitions.Action.exile (topSlice 10),
      putOntoBattlefield (allFromAmong isArtifact them) ]
theorem okTezzeretAllArtifacts : Action.check (DRS.ofList []) tezzeretAllArtifacts = [] := by decide

def companyContext : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) (Semantics.Templates.Action.lookAt (topSlice 6))).1
def companyDescribedSlice : NounPhrase :=
  (match (Semantics.Cards.«Collected Company») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ (Action.sequentially (List.cons _ (List.cons (Action.move part _
        _ _ _) _)))) _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okCompanyDescribedSlice :
    NounPhrase.checkAt (some .object) companyContext companyDescribedSlice = [] := by decide
def companyBareSlice : NounPhrase := someOf (exactly 2) them
theorem okCompanyBareSlice : NounPhrase.checkAt (some .object) companyContext companyBareSlice =
    [] := by
  decide

/-- Lord of the Void -/
def exileTopThenPutFromAmong : Action :=
  sequentially
    [ Semantics.Definitions.Action.exile (slice .top 7 (libraryOf (target isOpponent))),
      putOntoBattlefieldUnderYourControl (fromAmong (exactly 1) isCreature them) ]
theorem okExileTopThenPutFromAmong : Action.check (DRS.ofList []) exileTopThenPutFromAmong = [] := by decide

/-- Thought Sponge -/
def greatestCardsAnOpponentDrew : Amount :=
  (match (Semantics.Cards.«Thought Sponge») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.static (StaticSpec.replace _ (Replacement.withRiders
        (List.cons (MoveRider.withCounters part _ _) _)) _ _ _)) _)) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okGreatestCardsAnOpponentDrew : Amount.check (DRS.ofList []) greatestCardsAnOpponentDrew = [] := by
  decide
def greatestCardsAPlayerDiscardedThisWay : Amount :=
  aggregateOver .max isPlayer
    (eventTally (Semantics.Subject.actor (they)) (discard (allOf (cardIn Semantics.Templates.Location.hand))) thisWay .events)
/-- Windfall: "Each player discards their hand, then draws cards equal to the greatest number of
cards a player discarded this way." -/
def windfallDiscardAndDraw : Action :=
  instruct eachPlayer
    (sequentially [discard (allOf (cardIn Semantics.Templates.Location.hand)), draw greatestCardsAPlayerDiscardedThisWay])
theorem okGreatestCardsAPlayerDiscardedThisWay :
    Action.check (DRS.ofList []) windfallDiscardAndDraw = [] := by decide
/-- Alone, "this way" reads no discard [CR#608.2c]. -/
theorem badGreatestCardsAPlayerDiscardedThisWayAlone :
    Amount.check (DRS.ofList []) greatestCardsAPlayerDiscardedThisWay =
      [.thisWayUngrounded] := by decide

def eachPlayerBase : NounPhrase := eachPlayer
theorem okEachPlayerBase : NounPhrase.checkAt (some .player) (DRS.ofList []) eachPlayerBase = [] := by decide
theorem eachPlayerBindsOneDistributedMember :
    let result := stepNounPhrase 512 (DRS.ofList []) .obj Bool.true eachPlayerBase
    result.2.1 = [0] ∧
      (result.1.findById? 0).map (fun ref => (ref.det, ref.plur, ref.kind)) =
        some (.each, .one, .player) := by decide
theorem eachPlayerDoesNotIntroduceASeparateGroup :
    countManys .player ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true eachPlayerBase).1) = 0 := by decide

/-- Soul Ransom -/
def soulRansomRansom : Action :=
  (match (Semantics.Cards.«Soul Ransom») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons (Ability.activated _ part _ _ _ _) _))) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okSoulRansomRansom : Action.check (DRS.ofList []) soulRansomRansom = [] := by decide
def thisAurasController : NounPhrase := (match (Semantics.Cards.«Soul Ransom») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons (Ability.activated _ (Action.sequentially (List.cons
        (Action.instruct part _) _)) _ _ _ _) _))) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okThisAurasController : NounPhrase.checkAt (some .player) (DRS.ofList []) thisAurasController = [] := by
  decide
def targetCreaturesController : NounPhrase := controllerOf (target isCreature)
theorem okTargetCreaturesController :
    NounPhrase.checkAt (some .player) (DRS.ofList []) targetCreaturesController = [] := by decide
theorem possessiveDeicticIsReadableByIt :
    countReach (.atSlot .permanent) .one ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true thisAurasController).1) = 1 := by decide
/-- "this Aura's controller": the possessor is the Source, a self-referent no demonstrative
reads ("that enchantment" finds nothing; `Determiner.self`, owner ruling 2026-10-24), while a
later "this Aura" re-mentions it, on the battlefield as an enchantment (`mentionThis`). -/
theorem possessiveDeicticSourceIsNoDemonstrativeAntecedent :
    let result := stepNounPhrase 512 (DRS.ofList []) .obj Bool.true thisAurasController
    let again := stepNounPhrase 512 result.1 .obj Bool.true thisAura
    result.2.1 = [1] ∧ resolveCandidates (.word enchantment) .one result.1 = [] ∧
      again.2.1 = [0] ∧
      (again.2.1.filterMap again.1.findById?).all
        (fun ref => ref.zone == some .battlefield && ref.ty == [.enchantment]) := by decide

theorem possessiveDescribedBaseUnchanged :
    countReach (.atSlot .permanent) .one ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true targetCreaturesController).1) = 1 := by decide

/-- Open the Vaults -/
def openTheVaults : Action :=
  returnToBattlefield
    (allOf (and [ isCard, or [isArtifact, isEnchantment],
                   in_ (graveyardOf (Semantics.Templates.NounPhrase.bare isPlayer)) ]))
    [under (ownerOf them)] (from_ := graveyardOf (Semantics.Templates.NounPhrase.bare isPlayer))
theorem okOpenTheVaults : Action.check (DRS.ofList []) openTheVaults = [] := by decide

/-- Codecracker Hound -/
def codecrackerHoundLook : Action :=
  sequentially
    [ Semantics.Templates.Action.lookAt (topSlice 2), putInto (someOf (exactly 1) them) Semantics.Templates.Location.hand,
      putInto theOther Semantics.Templates.Location.graveyard ]
theorem okCodecrackerHoundLook : Action.check (DRS.ofList []) codecrackerHoundLook = [] := by decide

def oneOfTheTopTwo : NounPhrase := someOf (exactly 1) (topSlice 2)
theorem okOneOfTheTopTwo : NounPhrase.checkAt (some .object) (DRS.ofList []) oneOfTheTopTwo = [] := by decide
def oneOfAnUncountedTop : NounPhrase :=
  someOf (exactly 1) (topSlice (countOf isCreature))
theorem okOneOfAnUncountedTop : NounPhrase.checkAt (some .object) (DRS.ofList []) oneOfAnUncountedTop = [] := by
  decide
def theArtifactsAmongTheTopFive : NounPhrase :=
  allFromAmong isArtifact (topSlice 5)
theorem okTheArtifactsAmongTheTopFive :
    NounPhrase.checkAt (some .object) (DRS.ofList []) theArtifactsAmongTheTopFive = [] := by decide
theorem theOtherAfterATwoCardLook : theOtherOk .object ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true oneOfTheTopTwo).1) = Bool.true := by
  decide
theorem theOtherNeedsAStatedCount :
    theOtherOk .object ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true oneOfAnUncountedTop).1) = Bool.false := by decide
theorem theRestStandsWhereTheOtherRefuses :
    theRestOk .object ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true oneOfAnUncountedTop).1) = Bool.true := by decide
theorem theOtherRefusesTheUniversalSlice :
    (theOtherOk .object ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true theArtifactsAmongTheTopFive).1),
     theRestOk .object ((stepNounPhrase 512 (DRS.ofList []) .obj Bool.true theArtifactsAmongTheTopFive).1)) = (Bool.false, Bool.true) := by
  decide

def eachPlayerShufflesTheirHandAndGraveyard : Action :=
  instruct eachPlayer (shuffleInto
    (both (allOf (cardIn (handOf they))) (allOf (cardIn (graveyardOf they)))))
theorem okEachPlayerShufflesTheirHandAndGraveyard :
    Action.check (DRS.ofList []) eachPlayerShufflesTheirHandAndGraveyard = [] := by decide

def unsummon : Action := returnToHand (target isCreature)
theorem okUnsummon : Action.check (DRS.ofList []) unsummon = [] := by decide
def grismold : Action :=
  instruct eachPlayer (create 1 (Semantics.TokenSpec.written ((creatureToken 1 1 [.green] [plant]))))
theorem okGrismold : Action.check (DRS.ofList []) grismold = [] := by decide
/-- Grenzo, Dungeon Warden -/
def grenzoBottomCard : Action := putInto bottomCard Semantics.Templates.Location.graveyard
theorem okGrenzoBottomCard : Action.check (DRS.ofList []) grenzoBottomCard = [] := by decide

theorem nekrataalTwoObjects : countOnes .object nekrataalRider = 2 := by decide
theorem bioplasmTestMintsNothing : countOnes .object bioplasmAfterTest = 2 := by decide

def wholeSliceAtBase : SliceCount := whole
theorem wholeSliceIsPluralAndUncounted :
    (NounPhrase.plur theArtifactsAmongTheTopFive, wholeSliceAtBase.exact) = (.many, none) := by
  decide
theorem describedSliceReadsAsCreature :
    NounPhrase.ty companyContext companyDescribedSlice = [.creature] := by decide
theorem bareSliceReadsUntyped : NounPhrase.ty companyContext companyBareSlice = [] := by decide
theorem describedSliceKeepsGroupZone :
    NounPhrase.zone companyContext companyDescribedSlice
      = NounPhrase.zone companyContext companyBareSlice := by
  decide

/-- Arcum Dagsson -/
def arcumDagssonSacrifice : Action :=
  controllerSacrifices (target (and [isArtifact, isCreature]))
theorem okArcumDagssonSacrifice : Action.check (DRS.ofList []) arcumDagssonSacrifice = [] := by decide

/-- Saw in Half: "Destroy target creature. If that creature dies this way, its controller creates
two tokens that are copies of that creature, except their power is half that creature's power and
their toughness is half that creature's toughness. Round up each time." The possessor "its" and
every later "that creature" read the destroyed creature: step `C` of `AC>R1` only prefers a
referent no same-clause pronoun took, and here no other creature remains. -/
def sawInHalfBody : Action :=
  (match (Semantics.Cards.«Saw in Half») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ part) _) => some part
      | _ => none
    | _ => none).get (by decide)
/-- The card checks clean, and the destroyed creature (`1`) is the only referent any anaphor
resolved: "its" and all three "that creature" mentions read it. -/
theorem okSawInHalf :
    Action.check (DRS.ofList []) sawInHalfBody = [] ∧
      (stepAction 64 .postEvent .obj (DRS.ofList []) sawInHalfBody).1.resolvedIds = [1] := by
  decide

/-- Zoyowa's Justice: "The owner of target artifact or creature with mana value 1 or greater
shuffles it into their library. Then that player discovers X, where X is its mana value." The
shuffle randomizes the card [CR#701.24a], so no later "it" names it as a card in the library, but
"its mana value" reads the permanent by its last known information, as it was moved out of a
public zone into a hidden one [CR#608.2h] (`DRS.shuffledLastKnown`). -/
def zoyowasJusticeBody : Action :=
  (match (Semantics.Cards.«Zoyowa's Justice») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.spell _ part) _) => some part
      | _ => none
    | _ => none).get (by decide)

/-- Gyome, Master Chef: "{1}, Sacrifice a Food: Target creature gains indestructible until
end of turn. Tap it." -/
def gyomeProtectionCost : Cost := compound [Semantics.Templates.Cost.mana [generic 1], perform (sacrifice (a (Semantics.Predicate.hasSubtype (food))))]
def gyomeProtectionBody : Action :=
  sequentially [gain (target isCreature) indestructible untilEndOfTurn, .setStatus .tapped it]
def gyomeProtection : Ability := activated gyomeProtectionCost gyomeProtectionBody

/-- The bare "it" resolves to the target creature (`2`), not the sacrificed cost. -/
theorem gyomeTapsItsTarget :
    Ability.check (DRS.ofList []) gyomeProtection = [] ∧
    (stepAction 64 .postEvent .obj
      (nextClause (stepCost 64 (DRS.ofList []) gyomeProtectionCost).1)
      gyomeProtectionBody).1.resolvedIds = [2] := by
  decide

/-- Prosperity Tycoon: "{2}, Sacrifice a token: This creature gains indestructible until end
of turn. Tap it." -/
def prosperityTycoonProtectionCost : Cost :=
  compound [Semantics.Templates.Cost.mana [generic 2], perform (sacrifice (a isToken))]
def prosperityTycoonProtectionBody : Action :=
  sequentially [gain thisCreature indestructible untilEndOfTurn, .setStatus .tapped it]
def prosperityTycoonProtection : Ability :=
  activated prosperityTycoonProtectionCost prosperityTycoonProtectionBody

/-- The bare "it" resolves to the source, Prosperity Tycoon (`2`), not the sacrificed cost. -/
theorem prosperityTycoonTapsItself :
    Ability.check (DRS.ofList []) prosperityTycoonProtection = [] ∧
    (stepAction 64 .postEvent .obj
      (nextClause (stepCost 64 (DRS.ofList []) prosperityTycoonProtectionCost).1)
      prosperityTycoonProtectionBody).1.resolvedIds = [2] := by
  decide

/-- Grimgrin, Corpse-Born: "Sacrifice another creature: Untap Grimgrin and put a +1/+1 counter
on it." -/
def grimgrinUntapCost : Cost := perform (sacrifice (a (and [isCreature, isOther])))
def grimgrinUntapBody : Action :=
  sequentially [untap thisCreature, putCounters 1 (Semantics.CounterKindSource.printed (p1p1Counter)) it]
def grimgrinUntap : Ability := activated grimgrinUntapCost grimgrinUntapBody

/-- The bare "it" resolves to the source, Grimgrin (`2`), not the sacrificed cost. -/
theorem grimgrinPutsTheCounterOnItself :
    Ability.check (DRS.ofList []) grimgrinUntap = [] ∧
    (stepAction 64 .postEvent .obj
      (nextClause (stepCost 64 (DRS.ofList []) grimgrinUntapCost).1)
      grimgrinUntapBody).1.resolvedIds = [2] := by
  decide

/-- The probes that found the defect: a later spell ability's "that creature", and its bare "it"
after an ability word, read an earlier spell ability's target. -/
theorem laterSpellAbilitiesReadEarlierTargets :
    Ability.checkText (DRS.ofList [])
        [spell (tap (target isCreature)), spell (untap (that creature))] = [] ∧
      Ability.checkText (DRS.ofList [])
        [spell (tap (target isCreature)), delirium (spell (untap it))] = [] := by decide

/-- Only targets pass on: an earlier spell ability's other mentions stay in it. -/
theorem laterSpellAbilitiesDoNotReadEarlierNonTargets :
    Ability.checkText (DRS.ofList [])
        [spell (tap (a isCreature)), spell (untap it)] = [.anaphor (.atSlot .permanent) .one 0] := by decide

/-- A triggered or activated ability's target is still no ordinary antecedent in a later ability,
which is linked to it at most [CR#607.1,607.2a]; and a spell's targets are its spell abilities'
alone, not a later ability of another kind's. -/
theorem otherAbilitiesTargetsStayTheirOwn :
    Ability.checkText (DRS.ofList [])
        [when (does (Semantics.Subject.actor (thisCreature)) enter) (tap (target isCreature)), spell (untap (that creature))] =
        [.anaphor (.word creature) .one 0] ∧
      Ability.checkText (DRS.ofList [])
        [activated tapSymbol (tap (target isCreature)),
         activated tapSymbol (untap (that creature))] =
        [.anaphor (.word creature) .one 0] ∧
      Ability.checkText (DRS.ofList [])
        [when (does (Semantics.Subject.actor (thisCreature)) enter) (tap (target isCreature)),
         at_ (beginningOfYour .endStep) (untap (that creature))] =
        [.anaphor (.word creature) .one 0] ∧
      Ability.checkText (DRS.ofList [])
        [spell (tap (target isCreature)),
         at_ (beginningOfYour .endStep) (untap (that creature))] =
        [.anaphor (.word creature) .one 0] := by decide

end Semantics.Cards

end
