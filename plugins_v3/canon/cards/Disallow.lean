card := {
  name := "Disallow",
  cost := [1, blue, blue],
  types := [instant],
  text := [spell (counter (target (or [isSpell, abilityHead anyActivated, abilityHead anyTriggered])))]
}
