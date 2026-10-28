/-- Expel the Interlopers -/
card := {
  name := "Expel the Interlopers",
  cost := [3, white, white],
  types := [sorcery],
  text := [
    spell
      [
        choose (a (qualityFrom number (number (fromTo 0 10)))),
        destroy (allOf [isCreature, hasPower atLeast chosenNumber])
      ]
  ]
}
