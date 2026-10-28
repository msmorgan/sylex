card := {
  name := "Temple Altisaur",
  cost := [4, white],
  types := [creature],
  subtypes := [dinosaur],
  text := [
    replace (would (a isSource) (dealDamage anyAmount (a [dinosaur, youControl, isOther])))
      (instead (preventDamage (allBut 1)))
  ],
  power := 3,
  toughness := 4
}
