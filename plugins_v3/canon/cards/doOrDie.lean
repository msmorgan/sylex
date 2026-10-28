/-- Do or Die -/
card := {
  name := "Do or Die",
  cost := [1, black],
  types := [sorcery],
  text := [
    spell
      [
        separateIntoPiles (allOf [isCreature, controlledBy (target isPlayer)]) 2,
        destroyCantRegenerate (allOf [isCreature, inPile (pileOfChoice they)])
      ]
  ]
}
