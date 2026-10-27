card riptideChronologist : Spelled :=
  { name := "Riptide Chronologist", cost := [generic 3, pip .blue, pip .blue],
    types := [.creature], subtypes := [human, wizard],
    text :=
      [ activated (compound [[pip .blue], sacrifice thisCreature])
          (untap (allOf (and [isCreature, ofYourChoice (.subtype .creature)]))) ],
    power := stat 1, toughness := stat 3 }
