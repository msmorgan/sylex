card := {
  name := "Beckoning Will-o'-Wisp",
  cost := [2, white],
  types := [creature],
  subtypes := [spirit],
  text := [
    flying,
    flavorWord "Lure the Unwary" (at_ (beginningOfYour combat) (choose anOpponent)),
    getsPt (allOf [isCreature, inCombat attackerOf (the isChosenPlayer)]) (up 1) (up 0)
  ],
  power := 1,
  toughness := 3
}
