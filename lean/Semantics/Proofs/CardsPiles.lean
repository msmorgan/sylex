import Semantics.Cards
import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Piles

Port of `idris/src/Experimental/Cards/Piles.idr`: the printed cards of the Piles family (votes,
piles, and the pronouns that read them), each a `Spelled`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- "two cards at random" from a hand. -/
def twoCardsAtRandom : NounPhrase := countedAtRandom (exactly 2) (cardIn hand)
/-- "two cards" from a hand, chosen. -/
def twoCardsChosen : NounPhrase := counted (exactly 2) (cardIn hand)

/-- The at-random mode changes nothing a later pronoun can read. -/
theorem atRandomModeIsAnnouncementNeutral :
    (stepNounPhrase 64 (DRS.ofList []) .obj true twoCardsAtRandom).1 =
      (stepNounPhrase 64 (DRS.ofList []) .obj true twoCardsChosen).1 := by
  rfl

/-- Truth or Consequences: the secret-council ability alone, as the Idris bench holds it. -/
def truthOrConsequencesVote : Ability :=
  secretCouncil
    (spell (sequentially
      [ instruct eachPlayer (vote .secretly (byLabel ["truth", "consequences"])),
        draw (votesFor "truth"),
        choose (aAtRandom isOpponent),
        instruct this_
          (dealDamage (times 3 (votesFor "consequences")) (that player)) ]))
theorem okTruthOrConsequencesVote : Ability.check (DRS.ofList []) truthOrConsequencesVote = [] := by decide

/-- Harness Infinity: the exchange action alone, as the Idris bench holds it. -/
def harnessInfinityExchange : Action := exchangeZones hand graveyard
theorem okHarnessInfinityExchange : Action.check (DRS.ofList []) harnessInfinityExchange = [] := by decide

theorem countedAtRandomIsPlural : NounPhrase.plur twoCardsAtRandom = .many := by decide

end Semantics.Cards
