card := {
  name := "Ghostly Keybearer",
  cost := [3, blue],
  types := [creature],
  subtypes := [spirit],
  text := [
    flying,
    whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some combat)))
      (unlock (doorOf (some locked) (targets (upTo 1) [room, youControl])))
  ],
  power := 3,
  toughness := 3
}
