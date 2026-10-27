/-- Enormous Energy Blade -/
card enormousEnergyBlade : Spelled :=
  { name := "Enormous Energy Blade", cost := [generic 2, pip .black], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt (equipped creature) (up 4) (up 0)),
        whenever (does any (attachment .attached thisEquipment (a isCreature)))
            (tap (that creature)),
        equip [generic 2] ] }
