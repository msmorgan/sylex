/-- Inquisitor's Flail -/
card := {
  name := "Inquisitor's Flail",
  cost := [2],
  types := [artifact],
  subtypes := [equipment],
  text := [
    replace (would (equipped creature) (dealDamage anyAmount (kind := some combat)))
      (instead (instruct it (dealDamage (times 2 thatMuch)))),
    replace
      (would (a [isCreature, isOther]) (dealDamage anyAmount (equipped creature) (some combat)))
      (instead (instruct it (dealDamage (times 2 thatMuch) (equipped creature)))),
    equip [2]
  ]
}
