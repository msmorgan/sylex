card := {
  name := "Dread Slaver",
  cost := [3, black, black],
  types := [creature],
  subtypes := [zombie, horror],
  text := [
    whenever
      (does (a [isCreature, did thisCreature (dealDamage anyAmount which) (within soFarThisTurn)])
         die)
      [
        returnToBattlefield it [(under you)],
        become (that creature) { colors := [black], subtypes := [zombie] }
      ]
  ],
  power := 3,
  toughness := 5
}
