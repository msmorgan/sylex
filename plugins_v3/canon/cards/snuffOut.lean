card := {
  name := "Snuff Out",
  cost := [3, black],
  types := [instant],
  text := [
    onlyWhile (altCost this_ (payLife 4)) (exists_ [isLand, swamp, youControl]),
    spell (destroyCantRegenerate (target [isCreature, non (hasColor black)]))
  ]
}
