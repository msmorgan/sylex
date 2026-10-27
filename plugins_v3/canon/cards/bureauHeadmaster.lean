card bureauHeadmaster : Spelled :=
  { name := "Bureau Headmaster", cost := [pip .red, pip .white], types := [.creature],
    subtypes := [human, assassin],
    text :=
      [ static (spellsCost (and [isSpell, equipment, did you (cast which) thisObject]) (less 1)),
        static (costShift (allOf (and [abilityHead (.keyword equip),
            did you (activate which) thisObject]))
            (less 1)) ],
    power := stat 2, toughness := stat 2 }
