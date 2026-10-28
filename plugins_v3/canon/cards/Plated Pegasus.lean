card := {
  name := "Plated Pegasus",
  cost := [2, white],
  types := [creature],
  subtypes := [pegasus],
  text := [
    flash,
    flying,
    replace (would (a isSpell) (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
      (instead (preventDamage (ofThatDamage 1)))
  ],
  power := 1,
  toughness := 2
}
