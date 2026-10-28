/-- Strionic Resonator -/
card := {
  name := "Strionic Resonator",
  cost := [2],
  types := [artifact],
  text := [
    activated (compound [[2], tapSymbol])
      [
        copyOnStack (target [abilityHead anyTriggered, youControl]),
        may (chooseNewTargets (that (copied ability)))
      ]
  ]
}
