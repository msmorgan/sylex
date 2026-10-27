/-- Bramble Elemental -/
card brambleElemental : Spelled :=
  { name := "Bramble Elemental", cost := [generic 3, pip .green, pip .green],
    types := [.creature], subtypes := [elemental],
    text :=
      [ whenever (does any (attachment .attached (a aura) thisCreature))
          (create 2 (creatureToken 1 1 [.green] [saproling])) ],
    power := stat 4, toughness := stat 4 }
