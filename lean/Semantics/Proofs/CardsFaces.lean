import Semantics.Cards
import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Faces

Port of `idris/src/Experimental/Cards/Faces.idr`: the printed cards of the Faces family (the
frames that carry more than one set: transforming, flip, adventurer, split, room, leveler,
prototype) and the phrase-level bench items beside them, each with an `ok…` theorem.

Not ported: `shapeshifterBox` and `tarmogoyfBox` (a printed `*` box is `none` under the
printed-star ruling; there is no printed-box term to write).
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- "Turn target creature face down. It's a 2/2 Cyberman artifact creature.": the listed
characteristics are the face-down spec [CR#708.2]. -/
def cyberConversion : Action :=
  turnFaceDown (target isCreature)
    { characteristics := some
        { types := [.artifact, .creature], subtypes := [cyberman],
          power := stat 2, toughness := stat 2 } }
theorem okCyberConversion : Action.check (DRS.ofList []) cyberConversion = [] := by decide

def breakOpen : Action :=
  turnFaceUp (target (and [isCreature, isFaceDown, anOpponentControls]))
theorem okBreakOpen : Action.check (DRS.ofList []) breakOpen = [] := by decide

def jushiApprentice : Ability :=
  activated (compound [[generic 2, pip .blue], tapSymbol])
    (sequentially
      [ draw 1,
        doIf (compareAmt (countOf (cardIn yourHand)) .atLeast 9)
          (flip thisCreature) ])
theorem okJushiApprentice : Ability.check (DRS.ofList []) jushiApprentice = [] := by decide

/-- Missy -/
def missyFaceDownReturn : Ability :=
  whenever (does (a (and [isCreature, non isArtifact, isOther])) die)
    (sequentially
      [ returnToBattlefield it [(faceDownAs
              { characteristics := some
                  { types := [.artifact, .creature], subtypes := [cyberman],
                    power := stat 2, toughness := stat 2 } }), (entersAs .tapped), (under you)] ])
theorem okMissyFaceDownReturn : Ability.check (DRS.ofList []) missyFaceDownReturn = [] := by decide

/-- Ral Zarek, Guest Lecturer's ultimate -/
def ralZarekGuestLecturerUltimate : Action :=
  sequentially
    [ flipCoins 5,
      instruct (target isOpponent) (skipPart .turn (letter .x)),
      define .x (coinsShowing .heads) ]
theorem okRalZarekGuestLecturerUltimate :
    Action.check (DRS.ofList []) ralZarekGuestLecturerUltimate = [] := by decide

def faceDownFlyingCounter : StaticSpec :=
  entersWithCounters (allOf (and [isCreature, youControl, isFaceDown]))
    1 flyingCounter
theorem okFaceDownFlyingCounter : StaticSpec.check (DRS.ofList []) faceDownFlyingCounter = [] := by decide

def goblinArchaeologist : Ability :=
  activated (compound [[pip .red], tapSymbol])
    (sequentially
      [ flipCoins 1,
        doIf (did you (flipCoins 1 (some .wins)) thisWay)
          (sequentially [destroy (target isArtifact), untap thisCreature]),
        doIf (did you (flipCoins 1 (some .loses)) thisWay) (sacrifice thisCreature) ])
theorem okGoblinArchaeologist : Ability.check (DRS.ofList []) goblinArchaeologist = [] := by decide

/-- Karplusan Minotaur's win arm -/
def karplusanMinotaurWinFlip : Ability :=
  whenever (does you (flipCoins anyAmount (some .wins)))
      (instruct thisCreature (dealDamage 1 (target isAnyTarget)))
theorem okKarplusanMinotaurWinFlip : Ability.check (DRS.ofList []) karplusanMinotaurWinFlip = [] := by decide

/-- Ral Zarek's ultimate -/
def ralZarekUltimate : Action :=
  sequentially [flipCoins 5, extraTurn (coinsShowing .heads)]
theorem okRalZarekUltimate : Action.check (DRS.ofList []) ralZarekUltimate = [] := by decide

/-- Krark's Thumb -/
def krarksThumbExtraFlip : Action :=
  replaceEvent (would you flipCoins)
    (instead (sequentially [flipCoins 2, ignoreOutcomes (chosenResults none 1)])) none
theorem okKrarksThumbExtraFlip : Action.check (DRS.ofList []) krarksThumbExtraFlip = [] := by decide

/-- Goblin Assassin -/
def goblinAssassinCoinTails : Action :=
  sequentially
    [ instruct eachPlayer (flipCoins 1),
      instruct (each (and [isPlayer, coinCameUp .tails])) (sacrifice (aTheirChoice isCreature)) ]
theorem okGoblinAssassinCoinTails : Action.check (DRS.ofList []) goblinAssassinCoinTails = [] := by decide

/-- Rakdos, the Showstopper -/
def rakdosShowstopperFlips : Action :=
  sequentially
    [ flipACoinForEach (each (and [ isCreature,
                            not (or [ demon,
                                        devil,
                                        imp ]) ])),
      destroy (each (and [isCreature, coinCameUp .tails])) ]
theorem okRakdosShowstopperFlips : Action.check (DRS.ofList []) rakdosShowstopperFlips = [] := by decide

def planeswalkerBackWithoutLoyalty : CardFace :=
  { characteristics :=
    { name := "", supertypes := [.legendary], types := [.planeswalker],
      subtypes := [arlinn] } }
theorem planeswalkerBackWithoutLoyaltyOk :
    CardFace.check .back planeswalkerBackWithoutLoyalty = [] := by decide

/-- Garruk Relentless -/
def garrukRelentlessFlip : Ability :=
  (match (cardNamed "Garruk Relentless // Garruk, the Veil-Cursed").card with
    | Card.transforming face _ => match face.characteristics.text with
      | (List.cons part _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okGarrukRelentlessFlip : Ability.check (DRS.ofList []) garrukRelentlessFlip = [] := by decide
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

/-- Mana Clash -/
def manaClashFlip : Action :=
  instruct (eachOf (youAnd (target isOpponent))) (flipCoins 1)
theorem okManaClashFlip : Action.check (DRS.ofList []) manaClashFlip = [] := by decide

/-- Kitsune Mystic -/
def kitsuneMysticFlip : Ability :=
  (match (cardNamed "Kitsune Mystic // Autumn-Tail, Kitsune Sage").card with
    | Card.flip face _ => match face.characteristics.text with
      | (List.cons part _) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okKitsuneMysticFlip : Ability.check (DRS.ofList []) kitsuneMysticFlip = [] := by decide

/-- Vesuvan Shapeshifter -/
def vesuvanShapeshifterCopyDuration : Action :=
  establishThroughout (copyChange thisCreature (that creature)
      [ alsoHas
          (at_ (beginningOfYour .upkeep)
            (may (turnFaceDown thisCreature faceDownCreature))) ])
    (untilEvent (does any (turnFaceDown thisCreature faceDownCreature)))
theorem okVesuvanShapeshifterCopyDuration :
    Action.check (DRS.ofList [{ det := .a, plur := .one, payload := .object [.creature] (some .battlefield) none none none }])
      vesuvanShapeshifterCopyDuration = [] := by
  decide

/-- "if a player cast two or more spells last turn": the greatest number of spells any one player
cast last turn is at least two. The count is per player: `aggregateOver .max isPlayer` takes each
player's own `eventCount` (their `they`) and the greatest of them, so two players casting one
spell each does not satisfy it. A `DidClause.ordinal` ("cast a second spell") names no per-player
period (`Period` has none), and a counted patient ("cast two or more spells") counts spells, not
casting events [CR#601.2a]. -/
def aPlayerCastTwoOrMoreLastTurn : Condition :=
  (match (cardNamed "Afflicted Deserter // Werewolf Ransacker").card with
    | Card.transforming _ face => match face.characteristics.text with
      | (List.cons _ (List.cons (Ability.triggered _ _ _ _ _ _ (Option.some part) _) _)) => some
        part
      | _ => none
    | _ => none).get (by decide)
theorem okAPlayerCastTwoOrMoreLastTurn :
    Condition.check (DRS.ofList []) aPlayerCastTwoOrMoreLastTurn = [] := by decide

/-- The per-player reading is the aggregate's domain binding: the same count with no domain to
bind `they` finds no player to read. -/
theorem badEventCountWithoutPlayerDomain :
    Condition.check (DRS.ofList [])
      (compareAmt (eventCount (cast (a isSpell)) they lastTurn) .atLeast 2) =
      [.anaphor (.word .player) .one 0] := by decide

/-- A counted patient of a casting refuses: one casting casts one spell [CR#601.2a]. -/
theorem badCountedCastPatientLastTurn :
    Condition.check (DRS.ofList [])
      (did (a isPlayer) (cast (counted (orMore 2) isSpell)) lastTurn) = [.singular] := by decide

/-- Chittering Host's face, as both melding cards carry it. -/
def chitteringHost : CardFace :=
  (match (cardNamed "Midnight Scavengers // Chittering Host").card with
    | (Card.transforming _ part) => some part
    | _ => none).get (by decide)
def chitteringHostOnScavengers : CardFace := (match (cardNamed "Midnight Scavengers // Chittering Host").card with
    | (Card.transforming _ part) => some part
    | _ => none).get (by decide)
theorem okChitteringHostOnScavengers : CardFace.check .back chitteringHostOnScavengers = [] := by
  decide
/-- Chittering Host as the GRAF RATS card carries it -/
def chitteringHostOnGrafRats : CardFace := (match (cardNamed "Midnight Scavengers // Chittering Host").card with
    | (Card.transforming _ part) => some part
    | _ => none).get (by decide)
theorem okChitteringHostOnGrafRats : CardFace.check .back chitteringHostOnGrafRats = [] := by
  decide

def meldThemInto : Action :=
  sequentially
    [ exile
        (allOf (and [ isCreature, youControl,
                       or [ named "Graf Rats",
                             named "Midnight Scavengers" ] ])),
      meldInto them "Chittering Host" ]
theorem okMeldThemInto : Action.check (DRS.ofList []) meldThemInto = [] := by decide

/-- Titania, Voice of Gaea: "At the beginning of your upkeep, if there are four or more land cards
in your graveyard and you both own and control Titania, Voice of Gaea and a land named Argoth,
Sanctum of Nature, exile them, then meld them into Titania, Gaea Incarnate." The coordinated
subject of the property test holds an indefinite conjunct, read existentially, and publishes its
group (owner ruling 2026-10-22). -/
def titaniaVoiceOfGaeaMeld : Ability :=
  at_ (beginningOfYour .upkeep)
    (if_ := some (and
      [ thereAre (counted (orMore 4) (and [isLand, cardIn yourGraveyard])),
        is (both thisCreature (a (and [isLand, named "Argoth, Sanctum of Nature"])))
          (and [youOwn, youControl]) ]))
    (sequentially [exile them, meldInto them "Titania, Gaea Incarnate"])
theorem okTitaniaVoiceOfGaeaMeld :
    Ability.check (DRS.ofList []) titaniaVoiceOfGaeaMeld = [] := by decide

/-- In *Titania, Voice of Gaea*, "them" reads the group of Titania and the land named Argoth
(referent 4), not the land cards counted in the graveyard. -/
theorem titaniaThemReadsTitaniaAndArgoth :
    let condition : Condition := and
      [ thereAre (counted (orMore 4) (and [isLand, cardIn yourGraveyard])),
        is (both thisCreature (a (and [isLand, named "Argoth, Sanctum of Nature"])))
          (and [youOwn, youControl]) ]
    let after := nextClause (stepCondition 512 (DRS.ofList []) condition).1
    (resolveCandidates .bare .many after).map (fun ref => (ref.id, ref.groupKey)) =
      [(4, some 4)] := by
  decide

end Semantics.Cards
