/-- Denry Klin, Editor in Chief -/
card denryKlin : Spelled :=
  { name := "Denry Klin, Editor in Chief", cost := [generic 2, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [cat, advisor],
    text :=
      [ static (enters thisCreature
          [withCounters 1
            (chosenKind [p1p1Counter, firstStrikeCounter, vigilanceCounter]) .fresh]),
        denryKlinSameKinds ],
    power := stat 2, toughness := stat 2 }
