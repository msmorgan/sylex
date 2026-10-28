card := {
  name := "Linvala, Keeper of Silence",
  cost := [2, white, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [angel],
  text := [
    flying,
    cant any
      (activate (allOf [abilityHead anyActivated, abilityOf (allOf [isCreature, opponentsControl])]))
  ],
  power := 3,
  toughness := 4
}
