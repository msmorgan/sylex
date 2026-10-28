/-- Iron Man, Bleeding Edge -/
card := {
  name := "Iron Man, Bleeding Edge",
  cost := [3, blue, blue],
  supertypes := [legendary],
  types := [artifact, creature],
  subtypes := [human, hero],
  text := [
    flying,
    whenever (does you (cast (a [isArtifact, isSpell])))
      (doOnlyOnceEachTurn (may (copyOnStack it (exceptions := [isntLegendary]))))
  ],
  power := 3,
  toughness := 5
}
