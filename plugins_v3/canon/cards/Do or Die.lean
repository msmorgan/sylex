/-- Pins separating creatures into piles and destroying the creatures in the chosen
pile. Each creature belongs to exactly one pile; the pile itself is not an Object
[CR#700.3a,700.3b]. -/
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
