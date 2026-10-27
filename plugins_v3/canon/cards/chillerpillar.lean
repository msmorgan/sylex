card chillerpillar : Spelled :=
  { name := "Chillerpillar", cost := [generic 3, pip .blue], supertypes := [.snow],
    types := [.creature], subtypes := [insect],
    text :=
      [ activated [generic 4, .snow, .snow] (monstrosity 2),
        static (onlyWhile (abilityGrant thisCreature flying)
          (is thisCreature (hasDesignation monstrous))) ],
    power := stat 3, toughness := stat 3 }
