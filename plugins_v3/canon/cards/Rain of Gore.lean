/-- Rain of Gore -/
card := {
  name := "Rain of Gore",
  cost := [black, red],
  types := [enchantment],
  text := [
    replace
      (causedBy (bySource (a (or [isSpell, abilityHead anyOnStack])))
         (would (controllerOf it) (gainLife anyAmount)))
      (instead (instruct (that player) (loseLife thatMuch)))
  ]
}
