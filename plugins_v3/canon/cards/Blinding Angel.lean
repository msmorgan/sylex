/-- Blinding Angel {3}{W}{W} — Creature — Angel 2/4. "Flying. Whenever Blinding Angel deals
combat damage to a player, that player skips their next combat phase." -/
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
