import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.TextChanges

A Text Change is a specified rewrite of an object's rules text, applied in layer 3
[CR#612.1,613.1c]; cleave's Text Change removes all text found within square brackets
[CR#702.148a,702.148b].
A Text Change is checked by kinds only: the rewritten text is not checked again.
Bracketed text awaits `syl-cleave-bracketed-text`.
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.ManaSymbol (generic)

open Semantics.Templates.Action (target)
open Semantics.Templates.CostShift (generic)

namespace Semantics.Proofs.TextChanges

/-- "{1}{U}{U}" -/
def cleaveCost : Cost := .mana [generic 1, pip .blue, pip .blue]

/-- Cleave's Text Change on a spell that prints no brackets removes nothing and still checks. -/
theorem okRemoveBracketedWithoutBrackets :
    Ability.check (DRS.ofList []) (.static (textChange thisSpell removeBracketed)) = [] := by decide

/-- "Change the text of target permanent by replacing all instances of one basic land type with
another." [CR#612.2] -/
theorem okReplaceLandTypeWord :
    Action.check (DRS.ofList [])
      (.establish (textChange (target isPermanent) (replaceWord [.landType])) none) = [] := by
  decide

/-- Artificial Evolution: "Change the text of target spell or permanent by replacing all instances
of one creature type with another." A creature type word used as a creature type is replaced
like a color word or a land type [CR#612.2]. -/
theorem okReplaceCreatureTypeWord :
    Action.check (DRS.ofList [])
      (.establish (textChange (target (.or [isSpell, isPermanent]))
        (replaceWord [.creatureType])) none) = [] := by
  decide

/-- New Blood: "Change the text of that creature by replacing all instances of one creature type
with Vampire." [CR#612.2] -/
theorem okReplaceCreatureTypeWithVampire :
    Action.check (DRS.ofList [])
      (.establish (textChange (target isCreature)
        (replaceWord [.creatureType] none (some (.subtype vampire)))) none) = [] := by
  decide

/-- "… one creature type with Swamp": a land type is not a creature type. -/
theorem badReplaceCreatureTypeWithLandType :
    Action.check (DRS.ofList [])
      (.establish (textChange (target isCreature)
        (replaceWord [.creatureType] none (some (.subtype swamp)))) none)
      = [.textWordSort [.subtype .creature] (.subtype .land)] := by decide

/-- "… one color word with Swamp": a replaced word of another sort than the row's is refused. -/
theorem badReplaceWordSort :
    Action.check (DRS.ofList [])
      (.establish (textChange (target isPermanent)
        (replaceWord [.color] none (some (.subtype swamp)))) none)
      = [.textWordSort [.color] (.subtype .land)] := by decide

/-- "Change the text of target spell or permanent by replacing all instances of one color word
with another or one basic land type with another." [CR#612.2] -/
theorem okReplaceColorOrLandTypeWord :
    Action.check (DRS.ofList [])
      (.establish (textChange (target (.or [isSpell, isPermanent]))
        (replaceWord [.color, .landType])) none) = [] := by decide

/-- "… one color word with Swamp": a word replaced by a word of another kind is refused. -/
theorem badReplaceAcrossKinds :
    TextChange.check (DRS.ofList []) (.replaceWord [.color, .landType] (some (.color .black))
      (some (.subtype swamp)))
      = [.kindMismatch (.quality .color) (.quality (.subtype .land))] := by decide

/-- Cleave's definition: the alternative cost, and the Text Change under its paid cost
[CR#702.148a]. -/
theorem okCleaveDefinition : Ability.check (DRS.ofList []) (cleave cleaveCost) = [] := by decide

/-- Overload's definition: the alternative cost, and "target" to "each" under its paid cost
[CR#702.96a]. -/
theorem okOverloadDefinition : Ability.check (DRS.ofList []) (overload cleaveCost) = [] := by decide

/-- Splice onto Arcane's definition: as you cast an Arcane spell, you may reveal this card; if you
do, that spell gains its text and you pay the cost as an additional cost [CR#702.47a]. -/
theorem okSpliceDefinition :
    Ability.check (DRS.ofList []) (Ability.splice (.hasSubtype arcane) cleaveCost) = [] := by
  decide

end Semantics.Proofs.TextChanges
