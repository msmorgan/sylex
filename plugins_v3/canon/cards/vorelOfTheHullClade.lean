/-- Vorel of the Hull Clade -/
card := {
  name := "Vorel of the Hull Clade",
  cost := [1, green, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, merfolk],
  text := [
    activated (compound [[green, blue], tapSymbol])
      (doubleCounters (target (or [isArtifact, isCreature, isLand])))
  ],
  power := 1,
  toughness := 4
}
