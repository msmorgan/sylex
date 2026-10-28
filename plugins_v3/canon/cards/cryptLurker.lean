card := {
  name := "Crypt Lurker",
  cost := [3, black],
  types := [creature],
  subtypes := [horror],
  text := [
    when (does thisCreature enter)
      (may (chooseOne (sacrifice (a isCreature)) (discard (a [isCreature, in_ yourHand]))) (draw 1))
  ],
  power := 3,
  toughness := 4
}
