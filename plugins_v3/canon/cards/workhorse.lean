card workhorse : Spelled :=
  { name := "Workhorse", cost := [generic 6], types := [.artifact, .creature],
    subtypes := [horse],
    text :=
      [ static (entersWithCounters thisCreature 4 p1p1Counter),
        activated
          (removeCounters (exactly 1) p1p1Counter thisCreature)
          (addMana 1 (manaOf [[.colorless]])) ],
    power := stat 0, toughness := stat 0 }
