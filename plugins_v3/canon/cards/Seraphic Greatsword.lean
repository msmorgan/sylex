card := {
  name := "Seraphic Greatsword",
  cost := [1, white],
  types := [artifact],
  subtypes := [equipment],
  text := [
    getsPt (equipped creature) (up 2) (up 2),
    whenever
      (does (equipped creature)
         (attack (a [isPlayer, superlative max (playerStat lifeTotal) isPlayer])))
      (createTappedAttacking 1
         {
           colors := [white],
           types := [creature],
           subtypes := [angel],
           text := [flying],
           power := 4,
           toughness := 4
         }
         (some (that player))),
    equip [4]
  ]
}
