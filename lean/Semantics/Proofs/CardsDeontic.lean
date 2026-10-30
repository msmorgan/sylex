import Semantics.Cards.Data.Fixtures
import Semantics.Check
import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsHelpers

-- Canon cards are `noncomputable` data (`Semantics.Cards.Data`); so is what reads them.
noncomputable section


/-!
# Semantics.Cards.Deontic

Port of `idris/src/Experimental/Cards/Deontic.idr`: the printed cards of the Deontic family
(can't, must, may, tolls, and their windows) and the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (a counted)
open Semantics.Templates.Predicate (and or not was quality isAttached attachedTo)
open Semantics.Templates.ManaSymbol (generic)
open Semantics.Templates.Location (and or not)
open Semantics.Templates.NounPhrase (and or)
open Semantics.Templates.Timing (not)
open Semantics.Templates.Replacement (doIf)

open Semantics.Definitions.CardType (creature)
open Semantics.Templates.Action (doIf target)
open Semantics.Templates.Condition (and isAttached not or was)
open Semantics.Templates.CostShift (generic)
open Semantics.Templates.DetPhrase (a)
open Semantics.Templates.MoveRider (attachedTo)
open Semantics.Templates.NounWord (creature)
open Semantics.Templates.QualityPayload (quality)
open Semantics.Templates.SliceCount (counted)

namespace Semantics.Cards

def infiltrate : Action := establishThroughout (cant any (block (target isCreature))) thisTurn
theorem okInfiltrate : Action.check (DRS.ofList []) infiltrate = [] := by decide
def changeOfHeart : Action := establishThroughout (cant ((Semantics.Subject.actor (target isCreature))) attack) thisTurn
theorem okChangeOfHeart : Action.check (DRS.ofList []) changeOfHeart = [] := by decide
def blindblast : Action :=
  sequentially
    [instruct this_ (dealDamage 1 (target isCreature)), establishThroughout
       (cant (Semantics.Subject.actor ((that creature))) block) thisTurn]
theorem okBlindblast : Action.check (DRS.ofList []) blindblast = [] := by decide
def blindingFlare : Action :=
  establishThroughout (cant (Semantics.Subject.actor ((targets anyNumber isCreature))) block) thisTurn
theorem okBlindingFlare : Action.check (DRS.ofList []) blindingFlare = [] := by decide
def cowardKiller : Action :=
  sequentially
    [ establishThroughout (cant ((Semantics.Subject.actor (target isCreature))) block) thisTurn,
      become (that creature) { characteristics := { subtypes := [coward] } }
        untilEndOfTurn ]
theorem okCowardKiller : Action.check (DRS.ofList []) cowardKiller = [] := by decide

/-- Auriok Siege Sled -/
def auriokSiegeSledDenial : Ability :=
  activated (Semantics.Cost.mana ([generic 1]))
    (establishThroughout (cant ((Semantics.Subject.actor (target (and [isArtifact, isCreature])))) (block thisCreature))
       thisTurn)
theorem okAuriokSiegeSledDenial : Ability.check (DRS.ofList []) auriokSiegeSledDenial = [] := by decide

/-- Blindblast -/
def blindblastWhole : Action :=
  sequentially
    [ instruct this_ (dealDamage 1 (target isCreature)), establishThroughout
        (cant (Semantics.Subject.actor ((that creature))) block) thisTurn,
      draw 1 ]
theorem okBlindblastWhole : Action.check (DRS.ofList []) blindblastWhole = [] := by decide

def sparkmagesGambit : Action :=
  sequentially
    [ instruct this_ (dealDamage 1 (eachOf (targets (upTo 2) isCreature))),
      establishThroughout (cant (Semantics.Subject.actor ((those creature))) block) thisTurn ]
theorem okSparkmagesGambit : Action.check (DRS.ofList []) sparkmagesGambit = [] := by decide

/-- Glacial Chasm -/
def glacialChasmCant : Ability :=
  (match (Semantics.Cards.«Glacial Chasm») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons _ (List.cons _ (List.cons part _))) => some part
      | _ => none
    | _ => none).get (by decide)
theorem okGlacialChasmCant : Ability.check (DRS.ofList []) glacialChasmCant = [] := by decide

def desperateCastaways : Ability :=
  static (onlyUnless (cant (Semantics.Subject.actor (thisCreature)) attack)
    (exists_ (and [isArtifact, youControl])))
theorem okDesperateCastaways : Ability.check (DRS.ofList []) desperateCastaways = [] := by decide

def munghaWurm : Ability :=
  static (cant (Semantics.Subject.actor (you)) (untap (counted (moreThan 1) isLand)) (timing := duringYourUntapStep))
theorem okMunghaWurm : Ability.check (DRS.ofList []) munghaWurm = [] := by decide
def dampingField : Ability :=
  static
      (cant (Semantics.Subject.actor ((bare isPlayer))) (untap (counted (moreThan 1) isArtifact))
         (timing := duringTheirUntapSteps))
theorem okDampingField : Ability.check (DRS.ofList []) dampingField = [] := by decide
def smoke : Ability := static
    (cant (Semantics.Subject.actor ((bare isPlayer))) (untap (counted (moreThan 1) isCreature))
       (timing := duringTheirUntapSteps))
theorem okSmoke : Ability.check (DRS.ofList []) smoke = [] := by decide
def winterOrb : Ability :=
  static
      (onlyWhile
         (cant (Semantics.Subject.actor ((bare isPlayer))) (untap (counted (moreThan 1) isLand))
            (timing := duringTheirUntapSteps))
    (is thisArtifact isUntapped))
theorem okWinterOrb : Ability.check (DRS.ofList []) winterOrb = [] := by decide
def staticOrb : Ability :=
  static
      (onlyWhile
         (cant (Semantics.Subject.actor ((bare isPlayer))) (untap (counted (moreThan 2) isPermanent))
            (timing := duringTheirUntapSteps))
    (is thisArtifact isUntapped))
theorem okStaticOrb : Ability.check (DRS.ofList []) staticOrb = [] := by decide

def winterMoon : Ability :=
  static
      (cant (Semantics.Subject.actor ((bare isPlayer)))
         (untap (counted (moreThan 1) (and [isLand, non (hasSupertype .basic)])))
      (timing := duringTheirUntapSteps))
theorem okWinterMoon : Ability.check (DRS.ofList []) winterMoon = [] := by decide

def leitmotifComposer : Ability :=
  activated (Semantics.Cost.mana ([generic 2, pip .blue]))
    (establishThroughout
       (cant any
          (block (allOf (and [isCreature, named (Semantics.NameSource.printed ("Leitmotif Composer"))])))) thisTurn)
theorem okLeitmotifComposer : Ability.check (DRS.ofList []) leitmotifComposer = [] := by decide

def berserkersOfBloodRidge : Ability :=
  static (must (Semantics.Subject.actor (thisCreature)) attack)
theorem okBerserkersOfBloodRidge : Ability.check (DRS.ofList []) berserkersOfBloodRidge = [] := by decide

def trumpetingArmodon : Ability :=
  activated (Semantics.Cost.mana ([generic 1, pip .green]))
    (establishThroughout (must ((Semantics.Subject.actor (target isCreature))) (block thisCreature)) thisTurn)
theorem okTrumpetingArmodon : Ability.check (DRS.ofList []) trumpetingArmodon = [] := by decide

def loathsomeCatoblepas : Ability :=
  activated (Semantics.Cost.mana ([generic 2, pip .green]))
    (establishThroughout (must any (block thisCreature)) thisTurn)
theorem okLoathsomeCatoblepas : Ability.check (DRS.ofList []) loathsomeCatoblepas = [] := by decide

def hipparion : Ability :=
  static
      (gatedBy (Semantics.Cost.mana ([generic 1])) (Semantics.Subject.actor (thisCreature))
         (block (allOf (and [isCreature, hasPower .atLeast 3]))))
theorem okHipparion : Ability.check (DRS.ofList []) hipparion = [] := by decide

def frodoBaggins : Ability :=
  static (onlyWhile (must any (block thisCreature))
    (is thisCreature (hasDesignation ringBearer you)))
theorem okFrodoBaggins : Ability.check (DRS.ofList []) frodoBaggins = [] := by decide

/-- Everybody Lives! -/
def everybodyLivesGateLine : Action :=
  establishThroughout (conjunction
                         [cant (Semantics.Subject.actor ((bare isPlayer))) loseGame, cant (Semantics.Subject.actor ((bare isPlayer)))
                            winGame]) thisTurn
theorem okEverybodyLivesGateLine : Action.check (DRS.ofList []) everybodyLivesGateLine = [] := by decide

/-- Nowhere to Run -/
def nowhereToRunWardLine : StaticSpec :=
  conjunction [ can (by_ (allOf (or [isSpell, abilityHead .anyOnStack]))) (target (allOf (and [isCreature, opponentsControl]))) (asThough := holds (is them (not (hasKeyword (Semantics.KeywordTerm.the hexproof.label))))),
      cant (Semantics.Subject.actor ((allOf (and [abilityHead (.keyword ward.label), abilityOf (those creature)]))))
          trigger ]
theorem okNowhereToRunWardLine : StaticSpec.check (DRS.ofList []) nowhereToRunWardLine = [] := by decide

/-- Mornsong Aria -/
def mornsongAriaLock : StaticSpec :=
  conjunction [cant (Semantics.Subject.actor ((bare isPlayer))) (draw anyAmount), cant (Semantics.Subject.actor ((bare isPlayer)))
                 (gainLife anyAmount)]
theorem okMornsongAriaLock : StaticSpec.check (DRS.ofList []) mornsongAriaLock = [] := by decide

def enkiraHostileScavenger : Ability :=
  static (onlyWhile (must any (block thisCreature))
    (is thisCreature (isAttached (some .equipped))))
theorem okEnkiraHostileScavenger : Ability.check (DRS.ofList []) enkiraHostileScavenger = [] := by decide

def anglerTurtle : Ability :=
  static (must (Semantics.Subject.actor ((allOf (and [isCreature, opponentsControl])))) attack)
theorem okAnglerTurtle : Ability.check (DRS.ofList []) anglerTurtle = [] := by decide

def whippoorwillImmunity : Action :=
  sequentially
    [ establishThroughout (cant any (regeneration (target isCreature))) thisTurn,
      establishThroughout
        (conjunction
          [ cant any
              (preventDamage allDamage
                (damage := would any (dealDamage anyAmount (that creature)))),
            cant any
              (redirectDamage allDamage (a (and [or [isPermanent, isPlayer], isOther]))
                (damage := would any (dealDamage anyAmount (that creature)))) ])
        thisTurn ]
theorem okWhippoorwillImmunity : Action.check (DRS.ofList []) whippoorwillImmunity = [] := by decide

/-- Failure // Comply -/
def complyNameLock : Action :=
  sequentially
    [ choose (a (quality .cardName)),
      establishThroughout (cant (Semantics.Subject.actor (yourOpponents))
                             (cast (bare (and [isSpell, named chosenName])))) untilYourNextTurn ]
theorem okComplyNameLock : Action.check (DRS.ofList []) complyNameLock = [] := by decide

/-- Academic Probation -/
def academicProbationNameMode : Action :=
  sequentially
    [ choose (a (qualityFrom .cardName (nameOfCard (non isLand)))),
      establishThroughout (cant (Semantics.Subject.actor (yourOpponents))
                             (cast (bare (and [isSpell, named chosenName])))) untilYourNextTurn ]
theorem okAcademicProbationNameMode : Action.check (DRS.ofList []) academicProbationNameMode = [] := by
  decide

def sandsOfTimeSkip : StaticSpec := partSkip eachPlayer .untapStep
theorem okSandsOfTimeSkip : StaticSpec.check (DRS.ofList []) sandsOfTimeSkip = [] := by decide

def eaterOfDaysSkip : Action := skipPart .turn 2
theorem okEaterOfDaysSkip : Action.check (DRS.ofList []) eaterOfDaysSkip = [] := by decide

/-- Battlefront Krushok -/
def battlefrontKrushokEvasion : Ability :=
  static (cant (by_ (counted (moreThan 1) isCreature)) (block thisCreature))
theorem okBattlefrontKrushokEvasion : Ability.check (DRS.ofList []) battlefrontKrushokEvasion = [] := by decide

/-- Peacekeeper -/
def peacekeeperCant : Ability :=
  static (cant (Semantics.Subject.actor ((allOf isCreature))) attack)
theorem okPeacekeeperCant : Ability.check (DRS.ofList []) peacekeeperCant = [] := by decide

/-- Blazing Archon -/
def blazingArchonCant : Ability :=
  static (cant (Semantics.Subject.actor ((allOf isCreature))) (attack you))
theorem okBlazingArchonCant : Ability.check (DRS.ofList []) blazingArchonCant = [] := by decide

/-- Canoptek Wraith -/
def canoptekWraith : Ability :=
  flavorWord "Wraith Form"
    (static (cant any (block thisCreature)))
theorem okCanoptekWraith : Ability.check (DRS.ofList []) canoptekWraith = [] := by decide

/-- Gadrak, the Crown-Scourge -/
def gadrakCantAttack : Ability :=
  static (onlyUnless (cant (Semantics.Subject.actor (thisCreature)) attack)
    (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 4))
theorem okGadrakCantAttack : Ability.check (DRS.ofList []) gadrakCantAttack = [] := by decide

/-- Berserker's Frenzy, the 1—14 striation -/
def berserkersFrenzyLowRoll : Action :=
  sequentially
    [ choose (counted anyNumber isCreature),
      establishThroughout (must (Semantics.Subject.actor (them)) block) thisTurn ]
theorem okBerserkersFrenzyLowRoll : Action.check (DRS.ofList []) berserkersFrenzyLowRoll = [] := by decide

/-- Damn -/
def damnDestroyLine : Action :=
  doUnderProhibition (destroy (target isCreature))
    (regeneration (a (and [isCreature, was (destroy which) thisWay])))
theorem okDamnDestroyLine : Action.check (DRS.ofList []) damnDestroyLine = [] := by decide

/-- Concussive Bolt, both paragraphs -/
def concussiveBolt : Action :=
  sequentially
    [ instruct this_ (dealDamage 4 targetPlayerOrPlaneswalker),
      doIf (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 3)
        (establishThroughout
           (cant (Semantics.Subject.actor ((allOf (and [isCreature, controlledBy splitOverPlaneswalker]))))
              block) thisTurn) ]
theorem okConcussiveBolt : Action.check (DRS.ofList []) concussiveBolt = [] := by decide

/-- "attack you unless their controller pays {2} for each" -/
def attackToll : StaticSpec :=
  (match (Semantics.Cards.«Ghostly Prison») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.static part) _) => some part
      | _ => none
    | _ => none).get (by decide)

/-- Archangel of Tithes -/
def archangelOfTithesBlockToll : Ability :=
  static (onlyWhile
    (gatedBy (scaledGenericMana (times 1 groupSize)) (Semantics.Subject.actor ((allOf isCreature))) block)
    (is thisCreature isAttacking))
theorem okArchangelOfTithesBlockToll : Ability.check (DRS.ofList []) archangelOfTithesBlockToll = [] := by decide
/-- Archangel of Tithes -/
def archangelOfTithesAttackToll : Ability :=
  static (onlyWhile
    (gatedBy (scaledGenericMana (times 1 groupSize)) (Semantics.Subject.actor ((allOf isCreature)))
       (attack (youOr (allOf (and [isPlaneswalker, youControl])))))
    (is thisCreature isUntapped))
theorem okArchangelOfTithesAttackToll : Ability.check (DRS.ofList []) archangelOfTithesAttackToll = [] := by
  decide

/-- Distortion Strike -/
def distortionStrikeLine : Action :=
  sequentially
    [ get (target isCreature) (up 1) (up 0) untilEndOfTurn,
      establishThroughout (cant any (block (that creature))) thisTurn ]
theorem okDistortionStrikeLine : Action.check (DRS.ofList []) distortionStrikeLine = [] := by decide

/-- Conqueror's Flail -/
def conquerorsFlailProhibition : StaticSpec :=
  onlyWhile
    (partScope .turn you
      (cant (Semantics.Subject.actor (yourOpponents)) (cast (bare isSpell))))
    (is this_ (attachedTo (a isCreature)))
theorem okConquerorsFlailProhibition : StaticSpec.check (DRS.ofList []) conquerorsFlailProhibition = [] := by
  decide

/-- Retrace's permission: the land card discarded is an additional cost of casting, not what is
cast, added to the mana cost or an alternative cost [CR#702.81a,601.2f,601.2h]. -/
def castFromGraveyardByDiscardingALand : StaticSpec :=
  can (Semantics.Subject.actor (you))
    (cast this_ (from_ := yourGraveyard)
      (payment := plusAdditional ((Semantics.Cost.perform (discard (a (and [isCard, isLand])))))))
theorem okCastFromGraveyardByDiscardingALand :
    StaticSpec.check (DRS.ofList []) castFromGraveyardByDiscardingALand = [] := by decide

/-- "Auras already attached to those artifacts" -/
def aurasAlreadyAttachedToThoseArtifacts : NounPhrase :=
  (match (Semantics.Cards.«Guardian Beast») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.static (StaticSpec.conditional (StaticSpec.conjunction _ (List.cons
        (StaticSpec.retention _ part) _)) _ _)) _) => some part
      | _ => none
    | _ => none).get (by decide)

/-- Guardian Beast's protection: "As long as this creature is untapped, noncreature artifacts you
control can't be enchanted, they have indestructible, and other players can't gain control of
them. This effect doesn't remove Auras already attached to those artifacts." The artifacts are the
subject the three clauses share; "can't be enchanted" forbids attaching an Aura to them, a
hypothetical indefinite the prohibition closes, so "they" reads the artifacts, and "can't gain control of" forbids the control change,
`gainControlOf`. The exemption is the "can't be enchanted" clause's alone: that clause makes an
Aura already on such an artifact illegal [CR#303.4c,704.5m], and "already attached" reads the
host back, as *Benevolent Blessing*'s does. -/
def guardianBeastProtection : StaticSpec :=
  (match (Semantics.Cards.«Guardian Beast») with
    | Card.singleFaced face => match face.characteristics.text with
      | (List.cons (Ability.static part) _) => some part
      | _ => none
    | _ => none).get (by decide)

/-- The exemption reaches "can't be enchanted" and not "have indestructible" or "can't gain
control of" (owner ruling 2026-10-22, 3b): it wraps that clause in the card, and wrapped around
the whole conjunction it is refused, a conjunction being no one clause that removes Attachments
and having no one host for the Auras to be attached to. -/
theorem guardianBeastExemptsOnlyTheEnchantProhibition :
    (match guardianBeastProtection with
     | .conditional (.conjunction _ [.retention (.cant .any _ _) _, .characteristicChange _ _,
         .cant _ _ _]) _ _ => true
     | _ => false) = true ∧
    StaticSpec.check (DRS.ofList [])
      (onlyWhile
        (retention
          (conjunction
            [ cant any (attachTo (a (Semantics.Predicate.hasSubtype (aura))) them),
              abilityGrant them indestructible,
              cant (Semantics.Subject.actor ((bare (and [isPlayer, isOther])))) (gainControlOf them) ]
            (bare (and [non isCreature, isArtifact, youControl])))
          aurasAlreadyAttachedToThoseArtifacts)
        (is thisCreature isUntapped)) = [.retentionClause, .retentionSubject] := by decide

/-- Fioran Reformist's Democracy reminder, "Players can't become the monarch." The card itself
is not spelled: the bench declares neither the Democracy designation nor "get rid of the
monarchy". -/
def fioranReformistPlayersCantBecomeTheMonarch : Ability :=
  static (cant (.actor (bare isPlayer)) (gainDesignation .actor monarch))
theorem okFioranReformistPlayersCantBecomeTheMonarch :
    Ability.check (DRS.ofList []) fioranReformistPlayersCantBecomeTheMonarch = [] := by decide

end Semantics.Cards

end
