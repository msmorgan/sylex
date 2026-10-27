/-- Asmira, Holy Avenger -/
card asmiraHolyAvenger : Spelled :=
  { name := "Asmira, Holy Avenger", cost := [generic 2, pip .green, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ flying,
        at_ (beginningOfPossessed .endStep eachPlayer)
          (putCounters
            (eventCount (putInto (a isCreature) yourGraveyard (from_ := battlefield)) any
               soFarThisTurn)
            p1p1Counter thisCreature) ],
    power := stat 2, toughness := stat 3 }
