/-- True-Name Nemesis -/
card trueNameNemesis : Spelled :=
  { name := "True-Name Nemesis", cost := [generic 1, pip .blue, pip .blue],
    types := [.creature], subtypes := [merfolk, rogue],
    text :=
      [ static (entersChoosingPlayer thisCreature (a isPlayer)),
        protection (objectsOfPlayer (the isChosenPlayer)) ],
    power := stat 3, toughness := stat 1 }
