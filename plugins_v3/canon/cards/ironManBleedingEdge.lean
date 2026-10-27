/-- Iron Man, Bleeding Edge -/
card ironManBleedingEdge : Spelled :=
  { name := "Iron Man, Bleeding Edge", cost := [generic 3, pip .blue, pip .blue],
    supertypes := [.legendary], types := [.artifact, .creature],
    subtypes := [human, hero],
    text :=
      [ flying,
        whenever (does you (cast (a
          (and [isArtifact, isSpell]))))
          (doOnlyOnceEachTurn (may (copyOnStack it (exceptions := [isntLegendary]))))
              ],
    power := stat 3, toughness := stat 5 }
