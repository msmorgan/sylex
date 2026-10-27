/-- Beckoning Will-o'-Wisp -/
card beckoningWillOWisp : Spelled :=
  { name := "Beckoning Will-o'-Wisp", cost := [generic 2, pip .white],
    types := [.creature], subtypes := [spirit],
    text :=
      [ flying,
        flavorWord "Lure the Unwary"
          (at_ (beginningOfYour .combat) (choose anOpponent)),
        static (getsPt (allOf (and [isCreature, inCombat .attackerOf (the isChosenPlayer)]))
          (up 1) (up 0)) ],
    power := stat 1, toughness := stat 3 }
