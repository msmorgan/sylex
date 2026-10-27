card clarionConqueror : Spelled :=
  { name := "Clarion Conqueror", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [dragon],
    text :=
      [ flying,
        static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (allOf (or [isArtifact, isCreature, isPlaneswalker])) ])))) ],
    power := stat 3, toughness := stat 3 }
