/-- Heat Wave -/
card := {
  name := "Heat Wave",
  cost := [2, red],
  types := [enchantment],
  text := [
    cumulativeUpkeep [red],
    cant (allOf [isCreature, hasColor blue]) (block (allOf [isCreature, youControl])),
    gatedBy (payLife (forEach 1 [isCreature, isBlocking, controlledBy they]))
      (actor (allOf [isCreature, non (hasColor blue)])) (block (allOf [isCreature, youControl]))
  ]
}
