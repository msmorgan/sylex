import Semantics.Cards.Data.Fixtures
import Semantics.Check
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsHelpers
import Semantics.Proofs.Helpers

/-!
# Semantics.Proofs.Keyword

Port of `idris/src/Experimental/Proofs/Keyword.idr`: the pins of the Keyword family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (a each the)
open Semantics.Templates.Predicate (and not did was)
open Semantics.Templates.ManaSymbol (generic)
open Semantics.Templates.Option.Amount (stat)
open Semantics.Templates.Location (and not)
open Semantics.Templates.NounPhrase (and)
open Semantics.Templates.Timing (not)

open Semantics.Templates.Action (target)
open Semantics.Templates.CharacteristicEdit (stat)
open Semantics.Templates.Condition (and did not was)
open Semantics.Templates.CostShift (generic)
open Semantics.Templates.DetPhrase (a each the)

namespace Semantics.Proofs.Keyword

theorem persistReturnsWithACounter :
    Ability.check (DRS.ofList []) persist = [] := by decide

theorem undyingReturnsWithACounter :
    Ability.check (DRS.ofList []) undying = [] := by decide

/-- "Sacrifice a creature: Draw a card." -/
theorem okSacrificeAsCost :
    Ability.check (DRS.ofList []) (activated (.perform (sacrifice (a isCreature))) (draw 1)) = [] := by
  decide

/-- "Target creature gains flying." -/
theorem okGainsKeyword :
    Action.check (DRS.ofList []) (gain (target isCreature) (keyword (.named "flying")) none) = [] := by decide

/-- "Creature spells you cast cost {1} less to cast." -/
theorem okCostSubjectOnStack :
    StaticSpec.check (DRS.ofList []) (.costShift (allOf (.and [isCreature, isSpell])) (.generic (.down 1) none)) = [] :=
        by
  decide

/-- "Creatures you control have flying." -/
theorem okBattlefieldFlying :
    StaticSpec.check (DRS.ofList []) (Templates.abilityGrant (allOf (and [isCreature, youControl])) (keyword (.named "flying"))) = [] := by
        decide

/-- "Target creature gets +1/+1, gains flying, and gains trample." -/
theorem okFlatCoordination :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [ Templates.modification (target isCreature) .power (.up 1),
          Templates.modification it .toughness (.up 1),
          Templates.abilityGrant it (keyword (.named "flying")),
          Templates.abilityGrant it (keyword (.named "trample")) ]) = [] := by
  decide

/-- "Target creature gets +1/+1 and gains flying and gains trample." -/
theorem badNestedCoordination :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [ .conjunction none
            [ Templates.modification (target isCreature) .power (.up 1),
              Templates.modification it .toughness (.up 1),
              Templates.abilityGrant it (keyword (.named "flying")) ],
          Templates.abilityGrant it (keyword (.named "trample")) ]) = [.notCoord] := by
  decide

/-- "Whenever a creature enters, destroy that creature." -/
theorem okThatCreatureAfterAntecedent :
    Ability.check (DRS.ofList [])
      (whenever (does ((Semantics.Subject.actor (a isCreature))) enter) (destroy (that (.type .creature))))
      = [] := by
  decide

/-- "This creature gets +1/+1 and it has flying.": "it" reads the Source; "that creature" never
does, the Source being a self-referent (`Determiner.self`; owner ruling 2026-10-24). -/
theorem staticSelfRemainsReadableByIt :
    let spec := fun (third : NounPhrase) => StaticSpec.conjunction none
      [Templates.modification thisCreature .power (.up 1),
        Templates.modification it .toughness (.up 1),
        Templates.abilityGrant third (keyword (.named "flying"))]
    let after := (stepStaticSpec 64 (DRS.ofList []) (spec it)).1
    Ability.check (DRS.ofList []) (.static (spec it)) = [] ∧
      Ability.check (DRS.ofList []) (.static (spec (that (.type .creature)))) =
        [.anaphor (.word (.type .creature)) .one 0] ∧
      (firstReach .bare .one (nextClause after)).map (·.id) = some 0 ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

/-- "Creatures you control get +1/+1 and they have flying." -/
theorem okCoordinatedPlural :
    Ability.check (DRS.ofList [])
      (.static
        (.conjunction none
          [ Templates.modification (allOf (and [isCreature, youControl])) .power (.up 1),
            Templates.modification them .toughness (.up 1),
            Templates.abilityGrant them (keyword (.named "flying")) ])) = [] := by
  decide

/-- "Enchanted creature gets +1/+1 and they have flying." -/
theorem badCoordinatedHostPlural :
    Ability.check (DRS.ofList [])
      (.static
        (.conjunction none
          [ Templates.modification (.attachHost .enchanted (.type .creature)) .power (.up 1),
            Templates.modification it .toughness (.up 1),
            Templates.abilityGrant them (keyword (.named "flying")) ])) = [.anaphor .bare .many 0] := by
  decide

/-- "Create a 1/1 white Soldier creature token with flying." -/
theorem okTokenKeywordAbility :
    Action.check (DRS.ofList [])
      (create 1
        (.written { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [soldier],
            power := some 1, toughness := some 1, text := [keyword (.named "flying")] } }))
      = [] := by
  decide

/-- "Creatures you control have '{T}: Draw a card.'" -/
theorem okQuotedGrantOnPermanent :
    StaticSpec.check (DRS.ofList []) (Templates.abilityGrant (allOf (and [isCreature, youControl])) (activated .tapSymbol (draw 1)))
      = [] := by
  decide

/-- "of the creature type of your choice" -/
theorem okYourChoiceCreatureType :
    Predicate.checkAt .object (DRS.ofList []) (.ofYourChoice (.subtype .creature) none) = [] := by decide

/-- "of the number of your choice" -/
theorem badYourChoiceNumber :
    Predicate.checkAt .object (DRS.ofList []) (.ofYourChoice .number none) = [.chosenQualityRead .number] := by
  decide

/-- "As this enchantment enters, choose a creature type." -/
def choosesCreatureType : Ability :=
  .static (entersChoosing thisEnchantment (.subtype .creature))

/-- "Creatures of the chosen type get +1/+1." -/
def chosenTypeGets : Ability :=
  .static
    (getsPt (allOf (.and [isCreature, ofChosen (.subtype .creature)])) (.up 1) (.up 1))

/-- "As this enchantment enters, choose a creature type. Creatures of the chosen type get
+1/+1." -/
theorem okReaderAfterChooser :
    Card.check (enchantmentWith [choosesCreatureType, chosenTypeGets]) = [] := by decide

theorem badReaderBeforeChooser :
    Card.check (enchantmentWith [chosenTypeGets, choosesCreatureType])
      = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

theorem badTwoChoosersOneSortRead :
    Card.check (enchantmentWith [choosesCreatureType, choosesCreatureType, chosenTypeGets])
      = [.choiceRef .theChoice (.quality (.subtype .creature)) 2] := by
  decide

theorem badChosenReadWrongSort :
    Card.check
      (enchantmentWith
        [ .static (entersChoosing thisEnchantment .color),
          chosenTypeGets ]) = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

theorem badChosenProtectionBeforeChoice :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.creature], subtypes := [angel],
            power := stat 2, toughness := stat 2,
            text :=
              [ .static
                  (Templates.abilityGrant thisCreature
                    (.keyword (.named "protection") [.quality (ofChosen .color)] [])),
                .static (entersChoosing thisCreature .color) ] } })
      = [.choiceRef .theChoice (.quality .color) 0] := by
  decide

theorem badAscribedQualityBeforeChoice :
    Card.check
      (enchantmentWith
        [ .static
            (qualityChange (allOf (.and [isCreature, .hasPossessor .controller .you])) .adds
              (.quality (.chosen (ofChosen (.subtype .creature))))),
          choosesCreatureType ]) = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

/-- "{T}: Draw a card. Activate only once each turn." -/
theorem okActivatedTurnLimit :
    Ability.check (DRS.ofList []) (.activated .tapSymbol (draw 1) none (some oncePerTurn)
        none none)
      = [] := by
  decide

/-- "As long as a card exiled with this creature has <k>, this creature has <k>." -/
def exiledHas (k : KeywordTerm) : StaticSpec :=
  .conditional (keywordGrant thisCreature k (.and [.in_ (.exiledWith thisCreature), .hasKeyword k]))
    (exists_ (.and [.in_ (.exiledWith thisCreature), .hasKeyword k])) .asLongAs

/-- `exiledHas` as the ability a line prints. -/
def whileExiledHas (k : KeywordTerm) : Ability := .static (exiledHas k)

/-- "As long as a card exiled with this creature has flying, this creature has flying." -/
def flyingWhileExiledFlying : Ability := whileExiledHas (.the (.named "flying"))

/-- "As long as a card exiled with this creature has flying, this creature has flying. The same is
true for menace and trample.": one static ability [CR#113.2c], its frame read for each keyword
[CR#702.1c], checked as the static ability whose parts are the three grants is. -/
theorem okKeywordListOverGrant :
    let keywords : List KeywordTerm :=
      [.the (.named "flying"), .the (.named "menace"), .the (.named "trample")]
    alsoForKeywords (whileExiledHas .thatKeyword) keywords =
        .sameIsTrueFor (whileExiledHas .thatKeyword) keywords ∧
      Ability.check (DRS.ofList []) (alsoForKeywords (whileExiledHas .thatKeyword) keywords) =
        Ability.check (DRS.ofList []) (.static (.conjunction none (keywords.map exiledHas))) ∧
      Ability.check (DRS.ofList []) (alsoForKeywords (whileExiledHas .thatKeyword) keywords) = [] :=
  ⟨rfl, by decide, by decide⟩

/-- An empty keyword list joins nothing, refused `nonEmpty`. -/
theorem badEmptyKeywordList :
    Ability.check (DRS.ofList []) (alsoForKeywords (whileExiledHas .thatKeyword) []) =
      [.nonEmpty] := by decide

/-- A frame neither triggered nor static has no one ability to join into, refused `nonEmpty`. -/
theorem badUnjoinableFrame :
    Ability.check (DRS.ofList [])
      (alsoForKeywords .mayBeginOnBattlefield [.the (.named "flying")]) = [.nonEmpty] := by
  decide

/-- `thatKeyword` outside a frame names no keyword, refused `knownKeywordTerm`. -/
theorem badThatKeywordOutsideAFrame :
    (Ability.check (DRS.ofList []) (whileExiledHas .thatKeyword)).contains .knownKeywordTerm :=
  by decide

/-- "the number of basic land types among lands you control" -/
theorem okBasicLandTypeAxis :
    Amount.check (DRS.ofList [])
      (.distinctCount (.subtype .land .basicOnly)
        (allOf (.and [isLand, .hasPossessor .controller .you]))) = [] := by
  decide

/-- "the number of basic creature types among creatures you control" -/
theorem badBasicCreatureTypeAxis :
    Amount.check (DRS.ofList []) (.distinctCount (.subtype .creature .basicOnly) (allOf (and [isCreature, youControl])))
      = [.kindAxisSort] := by
  decide

/-- "Echo {2}{W}" -/
theorem okParameterisedEcho :
    Ability.check (DRS.ofList []) (keywordCosting (.named "echo") (.mana [generic 2, pip .white])) = [] := by decide

/-- "{T}: This deals X damage to any target, where X is 3." -/
theorem okActivatedOpensItsOwnLetter :
    Ability.check (DRS.ofList [])
      (activated .tapSymbol
        (.sequentially
          [(Action.instruct .this (.dealDamage (.letter .x) (target isAnyTarget))), define .x 3])) = [] := by
  decide

/-- "{T}: … , where X is 3" -/
theorem badActivatedClosesCardLetter :
    Ability.check (costLetters (some [.variable])) (activated .tapSymbol (define .x 3))
      = [.openLetter .x] := by
  decide

theorem activatedManaCostIntroducesItsOwnX :
    Ability.check (costLetters (some [.variable]))
      (activated (.mana [.variable]) (Templates.instruct .you (draw (.letter .x)))) = [] := by
  decide

theorem sharedSubjectSurvivesSecondSingular :
    Action.check (DRS.ofList [])
      (.sequentially
        [ Semantics.Definitions.Action.exile (target isArtifact),
          establishFor (.firstMention "sharedSubject.subject" (target isCreature))
            [ Templates.modification (.laterMention "sharedSubject.subject" (target isCreature)) .power (.up 1),
              Templates.modification (.laterMention "sharedSubject.subject" (target isCreature)) .toughness (.up 1),
              Templates.abilityGrant (.laterMention "sharedSubject.subject" (target isCreature)) (keyword (.named "flying")) ]
            (some untilEndOfTurn) ]) = [] := by
  decide

/-- "target creature": the subject a shared-subject clause re-reads -/
theorem okSharedSubjectDelta :
    NounPhrase.checkAt (some .object) (nounResult (DRS.ofList []) (.firstMention "sharedSubject.subject" (target isCreature)))
      (.laterMention "sharedSubject.subject" (target isCreature)) = [] := by
  decide

theorem badSharedSubjectEmptyDelta :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.laterMention "sharedSubject.subject" it)
      = [.bindingIdentity "sharedSubject.subject"] := by
  decide

/-- "Pay 2 life: Draw a card." -/
theorem okOwnPayerCost : Ability.check (DRS.ofList []) (activated (Semantics.Cost.perform ((payLife 2))) (draw 1)) =
    [] := by
  decide

/-- "Counter target spell." -/
theorem okCounterSpell : Action.check (DRS.ofList []) (Semantics.Definitions.Action.counter (target isSpell)) = [] := by decide

/-- "Counter target creature or player." -/
theorem badCounterJoinedPlayer :
    Action.check (DRS.ofList []) (Semantics.Definitions.Action.counter (target isAnyTarget))
      = [.kindLte (.join .object .player) (.join .object .pile)] := by
  decide

/-- "Companion — Each permanent card in your starting deck has mana value 2 or less." -/
theorem okCompanionCharacteristicRead :
    Ability.check (DRS.ofList [])
      (companion
        (.everyCardIs isPermanentCard
          (.aCharacteristic (.compare [.stat .manaValue] .atMost 2)))) = [] := by
  decide

/-- "Companion — Each nonland card in your starting deck shares a card type." -/
theorem okCompanionSharedCardType :
    Ability.check (DRS.ofList []) (companion (.groupAgrees (.agree .cardType .share (allOf (.and [.not isLand, .isCard]))))) = [] := by
  decide

/-- "Companion — Each nonland card in your starting deck shares a color.": color is a
characteristic [CR#109.3] and the deck-building condition is checked against the starting deck
[CR#702.139a]. -/
theorem okCompanionSharedColor :
    Ability.check (DRS.ofList []) (companion (.groupAgrees (.agree .color .share (allOf (.and [.not isLand, .isCard]))))) = [] := by
  decide

/-- "Each player scries 1.": one scry clause over a distributed player reference
[CR#701.22a,701.22c]. -/
theorem okEachPlayerScriesOne :
    Action.check (DRS.ofList []) (Templates.instruct (each .isPlayer) (scry 1)) = [] := by decide

/-- "Fateseal 2.": the sorted library is an opponent's [CR#701.29a]. -/
theorem okFatesealAnOpponent :
    Action.check (DRS.ofList []) (fateseal 2) = [] := by decide

/-- "Choose flying or trample. This creature gains that ability until end of turn." -/
theorem okThatAbilityAfterChoice :
    Action.check (DRS.ofList [])
      (.sequentially
        [ choose (a (qualityFrom .ability (.abilitiesAmong [.the (.named "flying"), .the (.named "trample")]))),
          gain thisCreature (.thatAbility .theChoice) (some untilEndOfTurn) ]) = [] := by
  decide

/-- "This creature gains that ability until end of turn", with no ability chosen anywhere in
the text. -/
theorem badThatAbilityWithoutChoice :
    Action.check (DRS.ofList []) (gain thisCreature (.thatAbility .theChoice) (some untilEndOfTurn))
      = [.choiceRef .theChoice (.quality .ability) 0] := by
  decide

/-- "Hexproof from black": a hexproof ability with a quality [CR#702.11d]. -/
theorem okHexproofFrom : Ability.check (DRS.ofList []) (hexproofFrom (hasColor .black)) = [] := by
  decide

/-- A keyword ability reads as the keyword it is: "with flying" names flying's keyword, as the
written label would. -/
theorem hasKeywordOfAbility :
    hasKeyword (Semantics.KeywordTerm.the flying.label) = Predicate.hasKeyword (.the (.named "flying")) := by rfl

/-- So does each member of a keyword list: "flying, first strike, or trample". -/
theorem keywordListOfAbilities :
    (abilitiesAmong [(Semantics.KeywordTerm.the flying.label), (Semantics.KeywordTerm.the firstStrike.label), (Semantics.KeywordTerm.the trample.label)] : ChoiceDomain) =
      .abilitiesAmong [.the (.named "flying"), .the (.named "firstStrike"), .the (.named "trample")] := by
  rfl

/-- Madness {1}{R}: discarding this card exiles it instead, and its owner may cast it from exile
[CR#702.35a]. -/
theorem okMadness : Ability.check (DRS.ofList []) (madness (Semantics.Templates.Cost.mana [generic 1, pip .red])) = [] := by
  decide

/-- Madness's replacement body, "that player discards it, but exiles it instead of putting it
into their graveyard" [CR#702.35a]: discarding into exile leaves the card
both exiled, read by "the exiled card" and by the trigger's "when this card is exiled this way"
[CR#702.35a], and discarded, as effects referencing the discarded card find it [CR#702.35c]. -/
theorem madnessCardIsDiscardedAndExiled :
    let body : Action := discard thisCard .exile
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) body).1
    (participialCandidates (.deed Semantics.Definitions.Action.exile.deed) (some .one) none Bool.false after).length = 1 ∧
      (participialCandidates (.deed discard.deed) (some .one) none Bool.false after).length = 1 ∧
      Action.check (DRS.ofList [])
        (sequentially [body, putInto (the (was (Semantics.Definitions.Action.exile which))) Semantics.Templates.Location.graveyard]) = [] ∧
      body.namedDeed = some discard.deed ∧
      after.causedEvents.any (fun event => event.readBy (some (.deed discard.deed)) []) = Bool.true ∧
      after.causedEvents.any (fun event => event.readBy (some (.deed Semantics.Definitions.Action.exile.deed)) []) = Bool.true := by
  decide

/-- Mayhem {1}{R}: castable from your graveyard as long as you discarded this card this turn
[CR#702.187b]. -/
theorem okMayhem : Ability.check (DRS.ofList []) (mayhem (Semantics.Templates.Cost.mana [generic 1, pip .red])) = [] := by
  decide

/-- Umbra armor: enchanted permanent's destruction becomes the removal of its damage and this
Aura's destruction [CR#702.89a]. -/
theorem okUmbraArmor : Ability.check (DRS.ofList []) umbraArmor = [] := by decide

/-- Split second: players can't cast other spells or activate abilities that aren't mana
abilities [CR#702.61a]. -/
theorem okSplitSecond : Ability.check (DRS.ofList []) splitSecond = [] := by decide

/-- The Ring tempts you, its emblem counting the times it has [CR#701.54a,701.54c]. -/
theorem okTheRingTemptsYou : Action.check (DRS.ofList []) theRingTemptsYou = [] := by decide

/-! ### "The same is true for …" on the printed cards -/

/-- A card's line `i`. -/
private def lineOf (card : Card) (i : Nat) : Option Ability :=
  match card with
  | .singleFaced face => face.characteristics.text[i]?
  | _ => none

/-- The characteristic edits of each keyword's reading of a Same-Is-True-For line's frame, as the
checker reads them (`CharacteristicEdit.forKeyword`, `CharacteristicEdit.unfoldAll`): a static
frame's condition, or a triggered frame's conditional step. -/
private def partEdits : Ability → List (List CharacteristicEdit)
  | .sameIsTrueFor frame keywords =>
    let edits := match frame with
      | .static (.conditional (.characteristicChange _ edits) _ _) => edits
      | .triggered _ _ _ _ _ _ _ (.doIf _ (.establish (.characteristicChange _ edits) _) _ _) =>
        edits
      | _ => []
    keywords.map fun k => CharacteristicEdit.unfoldAll (edits.map (·.forKeyword (some k)))
  | _ => []

/-- How a part grants its keyword: `some (some t)` for the family term `t`, through
`addedSelectionOf` over all of the objects a predicate describes; `some none` for one bare keyword,
through `addedAbilities`; `none` for anything else, a family term inside `addedAbilities`
among it. -/
private def grantForm : List CharacteristicEdit → Option (Option KeywordTerm)
  | [.addedSelectionOf (.described .all _) (.specified [.term (.anyIn family)])] =>
    some (some (.anyIn family))
  | [.addedAbilities [.keyword _ [] []]] => some none
  | _ => none

/-- The grant form of each part of a card's line `i`. -/
private def grantForms (card : Card) (i : Nat) : List (Option (Option KeywordTerm)) :=
  ((lineOf card i).map partEdits).getD [] |>.map grantForm

/-- The frame and keywords of a card's Same-Is-True-For line `i`. -/
private def sameIsTrueForLine (card : Card) (i : Nat) : Option (Ability × List KeywordTerm) :=
  match lineOf card i with
  | some (.sameIsTrueFor frame keywords) => some (frame, keywords)
  | _ => none

/-- Required pin: *Death-Mask Duplicant* and *Cairn Wanderer* gain "all landwalk and protection
abilities" of the cards their conditions find (their rulings of 2004-12-01 and 2007-10-01): each
family term is granted through `addedSelectionOf` over those objects with a `.term (anyIn …)`
selection, each other keyword is granted bare, and no `addedAbilities` holds a family term
[CR#702.1c]. Death-Mask Duplicant's frame, read for landwalk, reads off exactly the objects its
condition finds. -/
theorem familyTermsGrantThroughAddedSelectionOf :
    grantForms (Semantics.Cards.«Death-Mask Duplicant») 1 =
        [ some none, some none, some none, some none, some none,
          some (some Cards.landwalkAbilities), some (some Cards.protectionAbilities), some none ] ∧
      grantForms (Semantics.Cards.«Cairn Wanderer») 1 =
        [ some none, some none, some none, some none, some none, some none,
          some (some Cards.landwalkAbilities), some none, some (some Cards.protectionAbilities),
          some none, some none, some none, some none ] ∧
      (sameIsTrueForLine (Semantics.Cards.«Death-Mask Duplicant») 1).map
          (fun (frame, keywords) => (frame, keywords[5]?)) =
        some (static (onlyWhile
          (keywordGrant thisCreature .thatKeyword
            (and [in_ (exiledWith thisCreature), hasKeyword .thatKeyword]))
          (exists_ (and [in_ (exiledWith thisCreature), hasKeyword .thatKeyword]))),
          some Cards.landwalkAbilities) ∧
      CharacteristicEdit.unfoldAll [.addedKeywordTerm Cards.landwalkAbilities
          (and [in_ (exiledWith thisCreature), hasKeyword Cards.landwalkAbilities])] =
        [.addedSelectionOf
          (allOf (and [in_ (exiledWith thisCreature), hasKeyword Cards.landwalkAbilities]))
          (.specified [.term Cards.landwalkAbilities])] :=
  ⟨by decide, by decide, rfl, rfl⟩

/-- Required pin: hexproof is a family term on *Odric, Lunarch Marshal*, *Bleeding Effect* and
*Urborg Scavengers*: "hexproof from [quality]" is a hexproof ability [CR#702.11d], so each grants
the hexproof variants the objects its condition finds have [CR#702.1c] (Odric's ruling of
2020-01-24), and every other keyword bare. -/
theorem odricGrantsHexproofVariants :
    (grantForms (Semantics.Cards.«Odric, Lunarch Marshal») 0).all Option.isSome ∧
      (grantForms (Semantics.Cards.«Odric, Lunarch Marshal») 0).filterMap (·.bind id) = [Cards.hexproofAbilities] ∧
      (grantForms (Semantics.Cards.«Bleeding Effect») 0).all Option.isSome ∧
      (grantForms (Semantics.Cards.«Bleeding Effect») 0).filterMap (·.bind id) = [Cards.hexproofAbilities] ∧
      (grantForms (Semantics.Cards.«Urborg Scavengers») 1).all Option.isSome ∧
      (grantForms (Semantics.Cards.«Urborg Scavengers») 1).filterMap (·.bind id) = [Cards.hexproofAbilities] := by
  decide

/-- The ability "the same is true for" made before the frame was a constructor: each keyword's
ability, joined into one: a triggered first part's trigger doing each part's effect in turn, or
a static ability whose parts are each part's; anything else joins nothing. -/
private def joinedParts : List Ability → Ability
  | parts@(.triggered event alternatives while_ joins timing limit intervening _ :: _) =>
    let actions := parts.filterMap fun
      | .triggered _ _ _ _ _ _ _ action => some action
      | _ => none
    if actions.length == parts.length then
      .triggered event alternatives while_ joins timing limit intervening (.sequentially actions)
    else .static (.conjunction none [])
  | parts@(.static _ :: _) =>
    let specs := parts.filterMap fun
      | .static spec => some spec
      | _ => none
    .static (.conjunction none (if specs.length == parts.length then specs else []))
  | _ => .static (.conjunction none [])

/-- *Odric, Lunarch Marshal*'s frame, for one keyword. -/
private def odricFor (k : KeywordTerm) : Ability :=
  at_ (beginningOfPossessed .combat eachPlayer)
    (doOnlyIf
      (establishThroughout
        (keywordGrant (allOf (and [isCreature, youControl])) k
          (and [isCreature, youControl, hasKeyword k]))
        untilEndOfTurn)
      (exists_ (and [isCreature, youControl, hasKeyword k])))

/-- *Odric, Lunarch Marshal*'s printed frame is `odricFor` read for `thatKeyword`. -/
theorem odricFrameIsOdricFor :
    (sameIsTrueForLine (Semantics.Cards.«Odric, Lunarch Marshal») 0).map (·.1) =
      some (odricFor .thatKeyword) := rfl

/-- Required pin: *Odric, Lunarch Marshal*'s frame, read for each of its keywords, checks as the
ability the keywords' own abilities join into did [CR#702.1c]: as a whole, and one keyword at a
time, flying granted itself and hexproof its variants. -/
theorem odricReadsAsItsJoinedKeywords :
    Ability.check (DRS.ofList [])
          (.sameIsTrueFor (odricFor .thatKeyword) [.the (.named "flying"), Cards.hexproofAbilities]) =
        Ability.check (DRS.ofList [])
          (joinedParts ([.the (.named "flying"), Cards.hexproofAbilities].map odricFor)) ∧
      Ability.check { DRS.ofList [] with thatKeyword := some Cards.hexproofAbilities }
          (odricFor .thatKeyword) =
        Ability.check (DRS.ofList []) (odricFor Cards.hexproofAbilities) ∧
      Ability.check { DRS.ofList [] with thatKeyword := some (.the (.named "flying")) }
          (odricFor .thatKeyword) =
        Ability.check (DRS.ofList []) (odricFor (.the (.named "flying"))) ∧
      ((Semantics.Cards.«Odric, Lunarch Marshal»).check = []) := by
  decide

/-- A keyword wants a modal spell when its own Definition writes a Mode Change: entwine,
escalate, spree and tiered [CR#702.42a,702.120a,702.172a,702.183a]. -/
theorem okEntwineEscalateSpreeTieredWantModes :
    [ entwine (.mana [generic 2]), escalate (.mana [generic 2]), spree, tiered ].all
      Ability.keywordWantsModes = Bool.true := by decide

/-- A keyword whose Definition writes no Mode Change wants none, a cost-bearing one included. -/
theorem kickerWantsNoModes : (kicker (.mana [generic 2])).keywordWantsModes = Bool.false := by decide

/-- Bloodthirst asks whether an opponent was dealt damage this turn [CR#702.54a]: the opponent is
the damage's recipient, and the agentless passive leaves the source that dealt it [CR#120.1]
unnamed. -/
theorem bloodthirstReadsTheOpponentAsTheDamagedPlayer :
    (match bloodthirst 1 with
      | .keyword _ _ [.static (.replace _ (.onlyIf c _) _ _ _)] => some c
      | _ => none) =
      some (Condition.was (dealDamage anyAmount anOpponent) (Semantics.LookbackSpan.duration (soFarThisTurn) none)) := rfl

/-- Reconfigure's second ability unattaches this permanent [CR#702.151a,701.3d]: the `unattach`
Template, stamped with no attach deed, since unattaching is not attaching. -/
theorem reconfigureUnattachesWithoutTheAttachDeed :
    (match reconfigure (.mana [generic 2]) with
      | .keyword _ _ [_, .activated _ effect _ _ _ _] => some effect
      | _ => none) = some (unattach thisPermanent) ∧
      (unattach thisPermanent).ownDeed? = none := ⟨rfl, rfl⟩

/-- Shadow's second half binds this creature as the blocker [CR#702.28b]: creatures without
shadow can't be blocked by it, not by every creature with shadow. -/
theorem shadowSecondHalfIsAboutThisCreature :
    (match shadow with
      | .keyword _ _ [_, .static (.cant subject action _)] => some (subject, action)
      | _ => none) =
      some (.by thisCreature,
        .block (allOf (and [isCreature, not (.hasKeyword (.the (.named "shadow")))]))) := rfl

/-- Melee counts opponents attacked with a creature this combat, not attack events [CR#702.121a]:
each axis gets one per such opponent. -/
theorem meleeCountsAttackedOpponents :
    let perOpponent : Amount := forEach 1 (and [isOpponent,
      did (Semantics.Subject.actor (you)) (.attackWith (a isCreature) (some .which)) (Semantics.LookbackSpan.duration (thisCombat) none)])
    (match melee with
      | .keyword _ _ [.triggered _ _ _ _ _ _ _
          (.establish (.characteristicChange _ [.stat _ (.up power), .stat _ (.up toughness)]) _)] =>
        some (power, toughness)
      | _ => none) = some (perOpponent, perOpponent) := rfl

/-- Time travel's exiled cards are suspended ones [CR#701.56a]: a suspended card has suspend as
well as a time counter [CR#702.62b]. -/
theorem timeTravelReadsSuspendedCards :
    (match timeTravel with
      | .ownScope _ (.sequentially (.choose _ (.described _ (.coord _ [_, exiled]) ) _ _ :: _)) =>
        some exiled
      | _ => none) =
      some (and [.isCard, .in_ exileZone, ownedBy .actor,
        .hasKeyword (.the (.named "suspend")), .hasCounters timeCounter]) := rfl

/-- The deed each step of a keyword's one activated ability's sequence performs. -/
private def activatedStepDeeds : Ability → List (Option Deed)
  | .keyword _ _ [.activated _ (.sequentially steps) _ _ _ _] => steps.map (·.ownDeed?)
  | _ => []

/-- Transmute and transfigure search and shuffle through the `search` and `shuffle` Definitions,
and aura swap exchanges through `exchangeCards`, so each step keeps its keyword action's deed
[CR#702.53a,702.71a,702.65a,701.23a,701.24a,701.12d]. -/
theorem keywordBodiesKeepTheirActionDeeds :
    activatedStepDeeds (transmute (.mana [generic 1])) =
        [some search.deed, some Semantics.Definitions.Action.reveal.deed, none, some shuffle.deed] ∧
      activatedStepDeeds (transfigure (.mana [generic 1])) =
        [some search.deed, none, some shuffle.deed] ∧
      (match auraSwap (.mana [generic 1]) with
        | .keyword _ _ [.activated _ (.withContinuation _ exchange _ _) _ _ _ _] =>
          exchange.ownDeed?
        | _ => none) = some exchangeCards.deed := by decide

end Semantics.Proofs.Keyword
