card sanctuaryBlade : Spelled :=
  { name := "Sanctuary Blade", cost := [generic 2], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (attachChoosing thisEquipment .color),
        static (conjunction [ getsPt (equipped creature) (up 2) (up 0),
            abilityGrant (equipped creature)
              (protection (ofTheLastChosen .color)) ]),
        equip [generic 3] ] }
