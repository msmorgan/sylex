card maro : Spelled :=
  { name := "Maro", cost := [generic 2, pip .green, pip .green], types := [.creature],
    subtypes := [elemental],
    text := [static (ptDefinition thisCreature .bothEach (countOf (cardIn yourHand)))] }
