card := {
  name := "Loaming Shaman",
  cost := [2, green],
  types := [creature],
  subtypes := [centaur, shaman],
  text := [
    when (does thisCreature enter)
      (instruct (target isPlayer) (shuffleInto (targets anyNumber (cardIn (graveyardOf they)))))
  ],
  power := 3,
  toughness := 2
}
