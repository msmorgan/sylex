import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Piles

Port of `idris/src/Experimental/Cards/Piles.idr`: the printed cards of the Piles family (votes,
piles, and the pronouns that read them), each a plain semantic value.
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (counted)

open Semantics.Templates.Ability (spell)
open Semantics.Templates.SliceCount (counted)

namespace Semantics.Cards

/-- "two cards at random" from a hand. -/
def twoCardsAtRandom : NounPhrase := countedAtRandom (exactly 2) (cardIn Semantics.Templates.Location.hand)
/-- "two cards" from a hand, chosen. -/
def twoCardsChosen : NounPhrase := counted (exactly 2) (cardIn Semantics.Templates.Location.hand)

/-- The at-random mode changes nothing a later pronoun can read. -/
theorem atRandomModeIsAnnouncementNeutral :
    (stepNounPhrase 64 (DRS.ofList []) .obj Bool.true twoCardsAtRandom).1 =
      (stepNounPhrase 64 (DRS.ofList []) .obj Bool.true twoCardsChosen).1 := by
  rfl

/-- Truth or Consequences: the secret-council ability alone, as the Idris bench holds it. -/
def truthOrConsequencesVote : Ability :=
  secretCouncil
    (spell (sequentially
      [ instruct eachPlayer (vote .secretly (byLabel ["truth", "consequences"])),
        draw (votesFor "truth"),
        choose (aAtRandom isOpponent),
        instruct this_
          (dealDamage (Semantics.Templates.Amount.times 3 (votesFor "consequences")) (that Semantics.Templates.NounWord.player)) ]))
theorem okTruthOrConsequencesVote : Ability.check (DRS.ofList []) truthOrConsequencesVote = [] := by decide

theorem countedAtRandomIsPlural : NounPhrase.plur twoCardsAtRandom = .many := by decide

end Semantics.Cards
