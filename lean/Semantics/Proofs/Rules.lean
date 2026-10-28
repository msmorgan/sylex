import Semantics.Definitions
import Semantics.Check.Rules

/-!
# Semantics.Proofs.Rules

The rules-table bench and its pins. Not a port of an Idris family — Idris never modelled the
tables — so the sentences here are the Comprehensive Rules' own, and the bench items are the
rows `plugins/builtin/rules` and `plugins/builtin/tokens` actually write.

Each `ok…` fixes the empty refusal list for a real row; each `bad…` fixes the complete list a
malformed one earns. `distinctPredefinedTokens` is asserted over the predefined-token catalog.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Rules

/-- Every designation row with inherent abilities, in declaration order. -/
def designationDefinitions : List RegistryDefinition := [initiative_definition, monarch_definition]

/-- The Saga and Siege rows exercised below, in their original declaration order. -/
def subtypeDefinitions : List RegistryDefinition := [siege_definition, saga_definition]

/-- Every card type's row, in the declared order (`cardTypes`). -/
def cardTypeDefinitions : List RegistryDefinition :=
  [ creature_definition, artifact_definition, land_definition, enchantment_definition,
    instant_definition, sorcery_definition,
    planeswalker_definition, battle_definition, kindred_definition, dungeon_definition ]

/-! ## State-based actions -/

/-- A scope binds objects or players; a color is neither. -/
theorem badSbaScopeKind :
    SbaRule.check
      { scope := .qualityNoun .color none,
        when := .compareAmt (toughnessOf .this) .atMost 0,
        then_ := putInto .this graveyard }
      = [.kindLte (.quality .color) (.join .object .player)] := by
  decide

/-! ## Conferrals -/

/-- A conferral scopes what can have an ability — an object, or a player granted one
[CR#113.1a,113.1b] — and a color is neither. -/
theorem badConferralScopeKind :
    ConferralRule.check
      { scope := .qualityNoun .color none, confer := .ability (keyword (.named "flying")) }
      = [.kindLte (.quality .color) (.join .object .player)] := by
  decide

theorem okPlottedCardCast : ConferralRule.check plottedCardCast = [] := by decide

theorem okForetoldCardLook : ConferralRule.check foretoldCardLook = [] := by decide

/-! ## The results of damage

Each result is a rule, conferred as a property on what the rules give it to, so no effect that
removes abilities switches it off; an `asEvent` does it as the damage is dealt [CR#120.4c]. A
replacement of a result replaces this row's action, not the damage: wither and infect replace the
marking and the life loss [CR#120.3d,702.80a,702.90b,702.90c]. Lifelink, an ability, is the
keyword's own body [CR#702.15b]. -/

theorem okPlayerDamageLifeLoss : ConferralRule.check playerDamageLifeLoss = [] := by decide

theorem okToxicPoison : ConferralRule.check toxicPoison = [] := by decide

/-- A creature with two toxic abilities has one result: each ability is the bare keyword, with
no body of its own, and the one row reads the total toxic value [CR#702.164b,702.164c]. -/
theorem toxicIsOneResultReadingTheTotal :
    (match toxic 2 with
      | .keyword (.named "toxic") [.number _] body => body.isEmpty
      | _ => false) = true ∧
      (match toxicPoison.confer with
        | .property (.asEvent _ (.putCounters count _ _)) => some count
        | _ => none) = some (totalToxicValue this_) := ⟨rfl, rfl⟩

/-- The results of damage to a planeswalker are no ability of it: they are a property the rules
confer, so an effect that makes a permanent lose all abilities leaves them [CR#120.3c,113.12]. -/
theorem planeswalkerDamageResultIsNotAnAbility :
    (match planeswalkerLoyaltyDamage with
      | .property _ => true
      | _ => false) = true := rfl

/-! ## The rules that read a keyword

A keyword whose rule the CR states as a rule of the game, not as an ability of its object, holds
no body (`lean/CONTRACTS.md`, "Keyword abilities with no body"); its rule is a row here, as
toxic's is. -/

theorem okDeathtouchDestruction : SbaRule.check deathtouchDestruction = [] := by decide

theorem okPhasingTurnBased : ConferralRule.check phasingTurnBased = [] := by decide

/-! ## Registry definitions -/

theorem okChargeCounter : RegistryDefinition.check chargeCounter_definition = [] := by decide

theorem okFlyingCounter : RegistryDefinition.check flyingCounter_definition = [] := by decide

/-- A keyword counter's conferral written with `grants` is the one `flyingCounter_definition`
writes, the keyword gained by its bearer [CR#122.1b,613.1f]. -/
theorem grantsIsTheKeywordConferral :
    flyingCounter_definition = .counter flyingCounter [grants (.named "flying")] := rfl

theorem p1p1CounterDefinitionZone :
    RegistryDefinition.check p1p1Counter_definition = [] := by decide

theorem m1m1CounterDefinitionZone :
    RegistryDefinition.check m1m1Counter_definition = [] := by decide

/-- [CR#122.1f] "If a player has ten or more poison counters, that player loses the game." A
poison counter is a player's. -/
theorem okPoisonCounter : RegistryDefinition.check (.counter (.named "poison" .player) []) = [] := by
  decide

/-- A counter is placed on an object or a player [CR#122.1] and on nothing else; a color is
neither. -/
theorem badCounterHolder :
    RegistryDefinition.check (.counter (.named "chargeCounter" (.quality .color)) [])
      = [.definitionHolder (.quality .color)] := by
  decide

/-- Counter names distinguish kinds [CR#122.1]. This registry uses nonempty names as lookup
keys, so it refuses an unnamed row. -/
theorem badNamelessCounter :
    RegistryDefinition.check (.counter (.named "") []) = [.definitionNamed] := by decide

theorem okGargoyle : RegistryDefinition.check gargoyle_definition = [] := by decide

/-- Subtypes are written as words [CR#205.3b]. The registry requires a nonempty word as its
lookup key. -/
theorem badNamelessSubtype :
    RegistryDefinition.check (.subtype (.of .creature "") []) = [.definitionNamed] := by decide

/-! ## The card types' rows

Each card type's declared `RegistryDefinition.cardType` row (`plugins_v3/builtin/definitions/card_type/`) checks,
and each rule a row confers is pinned on the row: it is one of the row's conferrals, and it checks.
-/

/-- The conferrals a registry definition carries. -/
def confersOf : RegistryDefinition → List Conferral
  | .counter _ confers | .subtype _ confers | .cardType _ confers | .designation _ confers =>
    confers
  | .relation _ => []

/-- [CR#302.1,508.1a,302.5,302.6,120.3e,704.5f,704.5g] A creature card may be cast as a
sorcery, a creature is a Combatant, it can attack and block, the summoning-sickness rule applies
to it, damage is marked on it, and the toughness and lethal-damage state-based actions apply to
it. -/
theorem okCreatureType : RegistryDefinition.check creature_definition = [] := by decide

/-- [CR#302.5] "Creatures can attack and block." -/
theorem okCreatureCanAttack :
    creatureCanAttack ∈ confersOf creature_definition ∧ creatureCanAttack.check = [] :=
  ⟨by simp [confersOf, creature_definition], by decide⟩
theorem okCreatureCanBlock :
    creatureCanBlock ∈ confersOf creature_definition ∧ creatureCanBlock.check = [] :=
  ⟨by simp [confersOf, creature_definition], by decide⟩

/-- [CR#302.6] "A creature can't attack unless it has been under its controller's control
continuously since their most recent turn began." -/
theorem okSummoningSicknessAttack :
    summoningSicknessAttack ∈ confersOf creature_definition ∧ summoningSicknessAttack.check = [] :=
  ⟨by simp [confersOf, creature_definition], by decide⟩

/-- [CR#302.6] "A creature's activated ability with the tap symbol or the untap symbol in its
activation cost can't be activated unless the creature has been under its controller's control
continuously since their most recent turn began." -/
theorem okSummoningSicknessActivation :
    summoningSicknessActivation ∈ confersOf creature_definition ∧
      summoningSicknessActivation.check = [] :=
  ⟨by simp [confersOf, creature_definition], by decide⟩

/-- [CR#302.6] The summoning-sickness reading's "their" reads the player its condition names, the
creature's controller: with no player named before it, "their most recent turn" reads nothing. -/
theorem badHeldSinceTheirTurnWithNoPlayer :
    Condition.check (DRS.ofList [])
      (heldSince (.matches this_ isCreature) (.beginningOf .turn none (.mostRecent they))) =
      [.anaphor (.word .player) .one 0] := by decide

/-- [CR#120.3e] Damage dealt to a creature by a source with neither wither nor infect is marked
on it. -/
theorem okCreatureDamageMarking :
    creatureDamageMarking ∈ confersOf creature_definition ∧ creatureDamageMarking.check = [] :=
  ⟨by simp [confersOf, creature_definition], by decide⟩

/-- [CR#704.5f] "If a creature has toughness 0 or less, it's put into its owner's graveyard." -/
theorem okToughnessZero :
    toughnessZero ∈ confersOf creature_definition ∧ toughnessZero.check = [] :=
  ⟨by simp [confersOf, creature_definition], by decide⟩

/-- [CR#704.5g] A creature with toughness greater than 0 and damage marked on it greater than or
equal to its toughness is destroyed. -/
theorem okLethalDamage :
    lethalDamage ∈ confersOf creature_definition ∧ lethalDamage.check = [] :=
  ⟨by simp [confersOf, creature_definition], by decide⟩

/-- [CR#306.1,306.5b,306.6,120.3c,704.5i] A planeswalker card may be cast as a sorcery; a
planeswalker enters with loyalty, can be attacked, loses loyalty to damage, and the loyalty
state-based action applies to it. -/
theorem okPlaneswalkerType : RegistryDefinition.check planeswalker_definition = [] := by decide

/-- [CR#306.5b] A planeswalker enters with loyalty counters equal to its printed loyalty, which
is the loyalty the row reads, the card not yet being on the battlefield [CR#306.5a]. -/
theorem okPlaneswalkerLoyalty :
    planeswalkerLoyalty ∈ confersOf planeswalker_definition ∧ planeswalkerLoyalty.check = [] :=
  ⟨by simp [confersOf, planeswalker_definition], by decide⟩

/-- [CR#306.6] "Planeswalkers can be attacked." -/
theorem okPlaneswalkerCanBeAttacked :
    planeswalkerCanBeAttacked ∈ confersOf planeswalker_definition ∧
      planeswalkerCanBeAttacked.check = [] :=
  ⟨by simp [confersOf, planeswalker_definition], by decide⟩

/-- [CR#120.3c] Damage dealt to a planeswalker removes that many loyalty counters from it. -/
theorem okPlaneswalkerLoyaltyDamage :
    planeswalkerLoyaltyDamage ∈ confersOf planeswalker_definition ∧
      planeswalkerLoyaltyDamage.check = [] :=
  ⟨by simp [confersOf, planeswalker_definition], by decide⟩

/-- [CR#704.5i] "If a planeswalker has loyalty 0, it's put into its owner's graveyard." -/
theorem okLoyaltyZero :
    loyaltyZero ∈ confersOf planeswalker_definition ∧ loyaltyZero.check = [] :=
  ⟨by simp [confersOf, planeswalker_definition], by decide⟩

/-- [CR#310.1,310.4b,310.5,120.3h,704.5w,310.9a] A battle card may be cast as a sorcery; a battle
enters with defense, can be attacked, loses defense to damage, the non-Siege state-based action
applies to it, and its controller chooses its protector as it enters. -/
theorem okBattleType : RegistryDefinition.check battle_definition = [] := by decide

/-- [CR#310.4b] A battle enters with defense counters equal to its printed defense, which is the
defense the row reads, the card not yet being on the battlefield [CR#310.4a]. -/
theorem okBattleDefense :
    battleDefense ∈ confersOf battle_definition ∧ battleDefense.check = [] :=
  ⟨by simp [confersOf, battle_definition], by decide⟩

/-- [CR#310.5] "Battles can be attacked." -/
theorem okBattleCanBeAttacked :
    battleCanBeAttacked ∈ confersOf battle_definition ∧ battleCanBeAttacked.check = [] :=
  ⟨by simp [confersOf, battle_definition], by decide⟩

/-- [CR#120.3h] Damage dealt to a battle removes that many defense counters from it. -/
theorem okBattleDefenseDamage :
    battleDefenseDamage ∈ confersOf battle_definition ∧ battleDefenseDamage.check = [] :=
  ⟨by simp [confersOf, battle_definition], by decide⟩

/-- [CR#704.5w,310.8] "If a non-Siege battle has defense 0, it's put into its owner's
graveyard." Its defense on the battlefield is its defense-counter count [CR#310.4c]. -/
theorem okBattleDefenseZero :
    battleDefenseZero ∈ confersOf battle_definition ∧ battleDefenseZero.check = [] :=
  ⟨by simp [confersOf, battle_definition], by decide⟩

/-- [CR#310.9a,205.3q] As a battle with no battle type enters, its controller chooses its
protector, and the only player it may choose is itself. -/
theorem okBattleProtector :
    battleProtector ∈ confersOf battle_definition ∧ battleProtector.check = [] :=
  ⟨by simp [confersOf, battle_definition], by decide⟩

/-- [CR#301.1] An artifact card may be cast as a sorcery. -/
theorem okArtifactType : RegistryDefinition.check artifact_definition = [] := by decide

/-- [CR#305.1] A land card may be played as a sorcery. -/
theorem okLandType : RegistryDefinition.check land_definition = [] := by decide

/-- [CR#303.1] An enchantment card may be cast as a sorcery. -/
theorem okEnchantmentType : RegistryDefinition.check enchantment_definition = [] := by decide

/-- [CR#304.1] An instant card may be cast whenever its owner has priority. -/
theorem okInstantType : RegistryDefinition.check instant_definition = [] := by decide

/-- [CR#307.1] A sorcery card may be cast as a sorcery. -/
theorem okSorceryType : RegistryDefinition.check sorcery_definition = [] := by decide

/-- [CR#308.1] Kindred confers nothing of its own. -/
theorem okKindredType : RegistryDefinition.check kindred_definition = [] := by decide

/-- [CR#309.2c] "Dungeon cards are not permanents. They can't be cast." -/
theorem okDungeonType : RegistryDefinition.check dungeon_definition = [] := by decide

/-- The rows are the declared card types, one each, in the declared order. -/
theorem cardTypeRowsAreDeclared :
    cardTypeDefinitions.map (fun
      | .cardType type _ => some type
      | _ => none) = cardTypes.map some := by decide

/-- A card type is written as a word [CR#205.2a]; the registry refuses a nameless one. -/
theorem badNamelessCardType :
    RegistryDefinition.check (.cardType { name := "", cardClass := some .permanentCard } []) =
      [.definitionNamed] := by
  decide

/-! ## The rules-defined subtypes (ruling 2026-10-10, `semantics-v2-subtype-rules`) -/

theorem okEquipmentDefinition : RegistryDefinition.check equipment_definition = [] := by decide

theorem okFortificationDefinition : RegistryDefinition.check fortification_definition = [] := by decide

theorem okAuraDefinition : RegistryDefinition.check aura_definition = [] := by decide

/-- [CR#714.3a,714.3c,714.4] The declared Saga row (`plugins_v3/builtin/definitions/subtype/`): its
lore counter on entering, the lore increment and the sacrifice rule. -/
theorem okSagaDefinition : RegistryDefinition.check saga_definition = [] := by decide

/-- [CR#310.12a,310.12b,704.5v] The declared Siege row (`plugins_v3/builtin/definitions/subtype/`): the
opponent protector, the last-counter trigger and the Siege state-based action. -/
theorem okSiegeDefinition : RegistryDefinition.check siege_definition = [] := by decide

/-- [CR#310.12a] As a Siege enters, its controller chooses its protector among their
opponents. -/
theorem okSiegeProtector :
    siegeProtector ∈ confersOf siege_definition ∧ siegeProtector.check = [] :=
  ⟨by simp [confersOf, siege_definition], by decide⟩

/-- [CR#310.12b] When the last defense counter is removed from a Siege, it is exiled, and then
its controller may cast it transformed without paying its mana cost. -/
theorem okSiegeLastCounter :
    siegeLastCounter ∈ confersOf siege_definition ∧ siegeLastCounter.check = [] :=
  ⟨by simp [confersOf, siege_definition], by decide⟩

/-- [CR#704.5v] A Siege with defense 0 that is the source of no ability that has triggered but
not yet left the stack is put into its owner's graveyard. -/
theorem okSiegeDefenseZero :
    siegeDefenseZero ∈ confersOf siege_definition ∧ siegeDefenseZero.check = [] :=
  ⟨by simp [confersOf, siege_definition], by decide⟩

/-- The lore increment is a turn-based action [CR#714.3c], a game action and no effect
[CR#703.1,609.1]: the `turnBased` conferral that encloses it says so, and the put itself carries
no cause. Doubling Season's "if an effect would put" is `causedBy .anEffect` [CR#614.16], so it
doubles a Saga's entry counter, put by a replacement effect [CR#714.3a], and not the increment.
The checker has no matcher, so this pins the row's spelling; the matching is the engine's. -/
theorem sagaLoreIncrementIsTurnBased :
    ∃ parts, saga_definition = .subtype Semantics.Definitions.saga parts ∧
      parts[1]? = some (.turnBased .firstMain (doIf sagaHasChapterAbilities sagaLoreIncrement)) :=
  ⟨_, rfl, rfl⟩

/-- The triggered-chapter guard reads the Saga's chapter abilities that have triggered and not yet
left the stack, waiting ones included [CR#714.4,704.5s,113.7]. -/
theorem okSagaHasTriggeredChapter :
    Condition.check (DRS.ofList []) sagaHasTriggeredChapter = [] := by
  decide

/-- "An ability that has triggered but not yet left the stack" is an ability, so it binds an
ability referent alone, as the Siege's guard writes it [CR#704.5v,310.7]. -/
theorem triggeredNotLeftStackSeedsAnAbility : triggeredNotLeftStack.seedsAbility = true := rfl

/-- The Siege's guard, "it isn't the source of an ability that has triggered but not yet left the
stack", over the row's own subject, checks
[CR#704.5v,310.7]. -/
theorem okSiegeGuard : Condition.check (DRS.ofList []) siegeHasNoWaitingTrigger = [] := by decide

/-! ## The designations' rows

A designation's row carries its inherent triggered abilities [CR#725.2,726.2], read with no
source, and controlled by the player who held the designation when each triggered
(`RegistryDefinition.controllingDesignation?`), an exception to [CR#113.8]. Each ability is pinned on its
row: it is one of the row's conferrals, and it checks in the row's context. -/

/-- [CR#725.1,725.2] "The monarch is a designation a player can have. There is no monarch in a
game until an effect instructs a player to become the monarch"; its two inherent triggered
abilities check. -/
theorem okMonarchDefinition : RegistryDefinition.check monarch_definition = [] := by decide

/-- [CR#725.2] "At the beginning of the monarch's end step, that player draws a card." -/
theorem okMonarchDraw :
    monarchDraw ∈ confersOf monarch_definition ∧
      monarchDraw.checkIn monarch_definition.conferralContext = [] :=
  ⟨by simp [confersOf, monarch_definition], by decide⟩

/-- [CR#725.2] "Whenever a creature deals combat damage to the monarch, its controller becomes
the monarch." -/
theorem okMonarchSteal :
    monarchSteal ∈ confersOf monarch_definition ∧
      monarchSteal.checkIn monarch_definition.conferralContext = [] :=
  ⟨by simp [confersOf, monarch_definition], by decide⟩

/-- [CR#726.1,726.2] The one of the initiative's inherent triggered
abilities the grammar can write checks; the two that venture into Undercity [CR#701.49d] wait on
a venture action (`lean-v3-venture-dungeon`). -/
theorem okInitiativeDefinition : RegistryDefinition.check initiative_definition = [] := by decide

/-- [CR#726.2] "Whenever one or more creatures a player controls deal combat damage to the player
who has the initiative, the controller of those creatures takes the initiative." -/
theorem okInitiativeSteal :
    initiativeSteal ∈ confersOf initiative_definition ∧
      initiativeSteal.checkIn initiative_definition.conferralContext = [] :=
  ⟨by simp [confersOf, initiative_definition], by decide⟩

/-- [CR#725.2,726.2] The designations' abilities are controlled by the player who held the
designation when they triggered; a card type's are its objects' own, controlled as their source
is [CR#113.8]. -/
theorem designationControllerReading :
    monarch_definition.controllingDesignation? = some monarch ∧
      initiative_definition.controllingDesignation? = some initiative ∧
      creature_definition.controllingDesignation? = none := by decide

/-- [CR#725.2] The monarch's abilities have no source, so a trigger on its row that names "this"
names nothing, and is refused. -/
theorem badSourceInDesignationAbility :
    RegistryDefinition.check
      (.designation monarch
        [.ability (at_ (beginningOfPossessed .endStep theMonarch) (sacrifice this_))])
      = [.noSource] := by decide

/-- The same text on a card names the card [CR#113.7]: the refusal is the designation's having no
source. -/
theorem okSourceInPrintedAbility :
    Ability.check (DRS.ofList [])
      (at_ (beginningOfPossessed .endStep theMonarch) (sacrifice this_)) = [] := by decide

/-- An ability a sourceless ability grants has a source of its own, the object it is granted to:
"this creature" in it names that creature. -/
theorem okSourceInGrantedAbility :
    RegistryDefinition.check
      (.designation monarch
        [.ability
          (at_ (beginningOfPossessed .endStep theMonarch)
            (gain (target isCreature)
              (at_ (beginningOfYour .upkeep) (sacrifice thisCreature)) untilEndOfTurn))])
      = [] := by decide

/-- The rows are declared designations, each once. -/
theorem designationRowsAreDeclared :
    designationDefinitions.map RegistryDefinition.controllingDesignation?
      = [some initiative, some monarch] := by decide

theorem okLeftHalfUnlocked : RegistryDefinition.check leftHalfUnlocked_definition = [] := by decide

/-- Goaded is one named designation [CR#701.15b]. This registry uses nonempty designation
names as lookup keys. -/
theorem badNamelessDesignation :
    RegistryDefinition.check (.designation (.named "" (.heldBy .player)) [])
      = [.definitionNamed] := by
  decide

/-! ## The predefined-token catalog -/

/-- [CR#111.10a] "A Treasure token is a colorless Treasure artifact token with '{T}, Sacrifice
this token: Add one mana of any color.'" -/
def treasure : PredefinedToken :=
  { name := "Treasure",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.treasure],
            text :=
              [ activated (.compound [.tapSymbol, .perform (sacrifice .this)])
                  (.addMana 1 (.anyColor .sameColor) []) ] } } }
theorem okTreasure : PredefinedToken.check treasure = [] := by decide

/-- [CR#111.10b] "A Food token is a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" -/
def food : PredefinedToken :=
  { name := "Food",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.food],
            text :=
              [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice .this)])
                  (gainLife 3) ] } } }
theorem okFood : PredefinedToken.check food = [] := by decide

/-- [CR#111.10c] "A Gold token is a colorless Gold artifact token with 'Sacrifice this token:
Add one mana of any color.'" -/
def gold : PredefinedToken :=
  { name := "Gold",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.gold],
            text :=
              [ activated (.perform (sacrifice .this))
                  (.addMana 1 (.anyColor .sameColor) []) ] } } }
theorem okGold : PredefinedToken.check gold = [] := by decide

/-- [CR#111.10f] "A Clue token is a colorless Clue artifact token with '{2}, Sacrifice this
token: Draw a card.'" -/
def clue : PredefinedToken :=
  { name := "Clue",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.clue],
            text :=
              [ activated (.compound [.mana [generic 2], .perform (sacrifice .this)])
                  (draw 1) ] } } }
theorem okClue : PredefinedToken.check clue = [] := by decide

/-- [CR#111.10g] "A Blood token is a colorless Blood artifact token with '{1}, {T}, Discard a
card, Sacrifice this token: Draw a card.'" -/
def blood : PredefinedToken :=
  { name := "Blood",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.blood],
            text :=
              [ activated
                  (.compound
                    [ .mana [generic 1], .tapSymbol, .perform (discard (a (.and [.isCard, .in_ yourHand]))),
                      .perform (sacrifice .this) ])
                  (draw 1) ] } } }
theorem okBlood : PredefinedToken.check blood = [] := by decide

/-- [CR#111.10h] "A Powerstone token is a colorless Powerstone artifact token with '{T}: Add
{C}. This mana can't be spent to cast a nonartifact spell.'" [CR#106.6] -/
def powerstone : PredefinedToken :=
  { name := "Powerstone",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.powerstone],
            text :=
              [ activated .tapSymbol
                  (.addMana 1 (.runs [[.colorless]])
                    [.spendNotOn [.toCast (.and [.not isArtifact, isSpell])]]) ] } } }
theorem okPowerstone : PredefinedToken.check powerstone = [] := by decide

/-- [CR#111.10w] "A Vibranium token is a colorless Vibranium artifact token with indestructible
and '{T}: Add {C}. This mana can't be spent to cast a nonartifact spell.'" -/
def vibranium : PredefinedToken :=
  { name := "Vibranium",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.vibranium],
            text :=
              [ keyword (.named "indestructible"),
                activated .tapSymbol
                  (.addMana 1 (.runs [[.colorless]])
                    [.spendNotOn [.toCast (.and [.not isArtifact, isSpell])]]) ] } } }
theorem okVibranium : PredefinedToken.check vibranium = [] := by decide

/-- The seven predefined tokens the builtin plugin carries. -/
def builtinPredefinedTokens : List PredefinedToken :=
  [blood, clue, food, gold, powerstone, treasure, vibranium]

theorem builtinPredefinedTokensCheck :
    builtinPredefinedTokens.flatMap PredefinedToken.check = [] := by decide

theorem builtinPredefinedTokensDistinct :
    distinctPredefinedTokens builtinPredefinedTokens = true := by decide

/-- An effect creates a predefined token by name [CR#111.10], so an unnamed entry is
unreachable. -/
theorem badUnnamedPredefinedToken :
    PredefinedToken.check { treasure with name := "" } = [.tokenNamed] := by decide

/-! ## The builtin tables -/

/-- The predicate-scoped rules tables the builtin plugin carries. What a card type or a subtype
confers, its results of damage and state-based actions among them, is on that type's declared
`RegistryDefinition` row (`plugins_v3/builtin/definitions/card_type/`, `plugins_v3/builtin/definitions/subtype/`),
not here. The deathtouch lethal-damage row [CR#704.5h] is not written yet. -/
def builtinTables : RulesTables :=
  { conferral := [plottedCardCast, foretoldCardLook, playerDamageLifeLoss, toxicPoison] }

theorem okBuiltinTables : RulesTables.check builtinTables = [] := by decide

end Semantics.Proofs.Rules
