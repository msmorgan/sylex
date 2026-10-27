card prismariTheInspiration : Spelled :=
  { name := "Prismari, the Inspiration", cost := [generic 5, pip .blue, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [elder, dragon],
    text :=
      [ flying,
        ward (payLife 5),
        static (abilityGrant (allOf (and [isInstantOrSorcery, isSpell,
            did you (cast which) thisObject])) storm) ],
    power := stat 7, toughness := stat 7 }
