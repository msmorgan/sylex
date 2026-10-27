/-- Archfiend's Vessel -/
card archfiendsVessel : Spelled :=
  { name := "Archfiend's Vessel", cost := [pip .black], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ lifelink,
        when (does thisCreature enter)
          (if_ := some (or
            [ did it (enter (from_ := yourGraveyard)) triggering,
              is it (did you (cast which (from_ := yourGraveyard)) thisObject) ]))
          (doIfDone (exile it)
            (some <| create 1 (written
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [demon],
                  text := [flying], power := stat 5, toughness := stat 5 } }))) ],
    power := stat 1, toughness := stat 1 }
