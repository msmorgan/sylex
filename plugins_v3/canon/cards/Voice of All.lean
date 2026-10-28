card := {
  name := "Voice of All",
  cost := [2, white, white],
  types := [creature],
  subtypes := [angel],
  text := [
    flying,
    entersChoosing thisCreature color,
    abilityGrant thisCreature (protection (ofChosen color))
  ],
  power := 2,
  toughness := 2
}
