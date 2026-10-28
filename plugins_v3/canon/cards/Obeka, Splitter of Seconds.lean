card := {
  name := "Obeka, Splitter of Seconds",
  cost := [1, blue, black, red],
  supertypes := [legendary],
  types := [creature],
  subtypes := [ogre, warlock],
  text := [
    menace,
    whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some combat)))
      (additionalStep upkeep none thatMuch)
  ],
  power := 2,
  toughness := 5
}
