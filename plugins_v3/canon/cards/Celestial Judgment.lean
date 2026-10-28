/-- Celestial Judgment -/
card := {
  name := "Celestial Judgment",
  cost := [4, white, white],
  types := [sorcery],
  text := [
    spell
      [
        doForEachKind (value power) (allOf isCreature) number
          (choose (a [isCreature, hasPower eq chosenNumber])),
        destroy (each [isCreature, notChosen])
      ]
  ]
}
