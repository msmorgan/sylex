/-- Realmwright -/
card realmwright : Spelled :=
  { name := "Realmwright", cost := [pip .blue], types := [.creature],
    subtypes := [vedalken, wizard],
    text :=
      [ static (entersChoosingFrom thisCreature (.subtype .land) basicTypesOnly),
        static (qualityChange (allOf (and [isLand, youControl])) .adds
          (chosenQuality (ofChosen (.subtype .land)))) ],
    power := stat 1, toughness := stat 1 }
