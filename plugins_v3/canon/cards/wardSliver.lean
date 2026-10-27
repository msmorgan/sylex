card wardSliver : Spelled :=
  { name := "Ward Sliver", cost := [generic 4, pip .white], types := [.creature],
    subtypes := [sliver],
    text :=
      [ static (entersChoosing thisCreature .color),
        static (abilityGrant (allOf sliver)
          (protection (ofChosen .color))) ],
    power := stat 2, toughness := stat 2 }
