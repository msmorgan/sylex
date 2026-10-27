card voiceOfAll : Spelled :=
  { name := "Voice of All", cost := [generic 2, pip .white, pip .white], types := [.creature],
    subtypes := [angel],
    text :=
      [ flying,
        static (entersChoosing thisCreature .color),
        static (abilityGrant thisCreature (protection (ofChosen .color))) ],
    power := stat 2, toughness := stat 2 }
