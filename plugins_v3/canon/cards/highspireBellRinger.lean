/-- Highspire Bell-Ringer -/
card highspireBellRinger : Spelled :=
  { name := "Highspire Bell-Ringer", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [djinn, monk],
    text :=
      [ flying,
        static (costShift
          (the (and [ isSpell,
                      did you (cast which) thisObject (ordinal := some (.nth 2, some .turn)) ]))
          (less 1)) ],
    power := stat 1, toughness := stat 4 }
