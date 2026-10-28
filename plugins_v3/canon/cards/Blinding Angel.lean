card := {
  name := "Blinding Angel",
  cost := [3, white, white],
  types := [creature],
  subtypes := [angel],
  text := [
    flying,
    whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some combat)))
      (instruct (that player) (skipPart combat 1))
  ],
  power := 2,
  toughness := 4
}
