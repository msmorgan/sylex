/-- Bloom Hulk -/
card bloomHulk : Spelled :=
  { name := "Bloom Hulk", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [plant, elemental],
    text := [when (does thisCreature enter) proliferate],
    power := stat 4, toughness := stat 4 }
