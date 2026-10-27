/-- Helica Glider -/
card helicaGlider : Spelled :=
  { name := "Helica Glider", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [nightmare, squirrel],
    text :=
      [ static (enters thisCreature
          [withCounters 1 (chosenKind [flyingCounter, firstStrikeCounter]) .fresh]) ],
    power := stat 2, toughness := stat 2 }
