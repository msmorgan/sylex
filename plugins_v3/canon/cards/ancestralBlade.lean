/-- Ancestral Blade -/
card ancestralBlade : Spelled :=
  { name := "Ancestral Blade", cost := [generic 1, pip .white], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ when (does thisEquipment enter)
          (sequentially
            [ create 1 (creatureToken 1 1 [.white] [soldier]),
              attachTo thisEquipment thatToken ]),
        static (getsPt (equipped creature) (up 1) (up 1)),
        equip [generic 1] ] }
