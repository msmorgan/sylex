card := {
  name := "Darksteel Garrison",
  cost := [2],
  types := [artifact],
  subtypes := [fortification],
  text := [
    abilityGrant (fortified land) indestructible,
    whenever (does (fortified land) (setStatus tapped theActor))
      (get (target isCreature) (up 1) (up 1) untilEndOfTurn),
    fortify [3]
  ]
}
