/-- Harried Spearguard -/
card harriedSpearguard : Spelled :=
  { name := "Harried Spearguard", cost := [pip .red], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ haste,
        when (does thisCreature die)
          (create 1 (written
            { characteristics :=
              { colors := [.black], types := [.creature], subtypes := [rat],
                text :=
                  [ static (cant thisToken block) ],
                power := stat 1, toughness := stat 1 } })) ],
    power := stat 1, toughness := stat 1 }
