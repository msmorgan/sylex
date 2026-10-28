card := {
  name := "Rhystic Study",
  cost := [2, blue],
  types := [enchantment],
  text := [
    whenever (does anOpponent (cast (a isSpell)))
      (instruct (that player) (doUnless (instruct you (may (draw 1))) [1]))
  ]
}
