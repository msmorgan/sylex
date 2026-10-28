/-- Verity Circle -/
card := {
  name := "Verity Circle",
  cost := [2, blue],
  types := [enchantment],
  text := [
    whenever (does (a [isCreature, anOpponentControls]) (setStatus tapped theActor))
      (if_ := is it (not (inCombat declaredAttacker))) (may (draw 1)),
    activated [4, blue] (tap (target [isCreature, not (hasKeyword flying)]))
  ]
}
