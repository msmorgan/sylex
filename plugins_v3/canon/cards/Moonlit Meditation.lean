card := {
  name := "Moonlit Meditation",
  cost := [2, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant [or [isArtifact, isCreature], youControl],
    replace
      (nthOccurrence (nth 1)
         (would (by_ you) (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) [])))
         (some turn))
      (instead (may (create groupSize (copyOf (enchanted permanent)) [])))
  ]
}
