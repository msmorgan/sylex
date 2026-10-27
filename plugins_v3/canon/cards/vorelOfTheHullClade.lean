/-- Vorel of the Hull Clade -/
card vorelOfTheHullClade : Spelled :=
  { name := "Vorel of the Hull Clade", cost := [generic 1, pip .green, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, merfolk],
    text :=
      [ activated (compound [[pip .green, pip .blue], tapSymbol])
          (doubleCounters (target (or [isArtifact, isCreature, isLand]))) ],
    power := stat 1, toughness := stat 4 }
