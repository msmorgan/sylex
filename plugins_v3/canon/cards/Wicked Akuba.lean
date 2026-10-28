card := {
  name := "Wicked Akuba",
  cost := [black, black],
  types := [creature],
  subtypes := [spirit],
  text := [
    activated [black]
      (instruct
         (target [isPlayer, did thisCreature (dealDamage anyAmount which) (within soFarThisTurn)])
         (loseLife 1))
  ],
  power := 2,
  toughness := 2
}
