/-- Mana Leak {1}{U} — Instant. "Counter target spell unless its controller pays {3}." -/
card := {
  name := "Mana Leak",
  cost := [1, blue],
  types := [instant],
  text := [spell (instruct (controllerOf (target isSpell)) (doUnless (instruct you (counter it)) [3]))]
}
