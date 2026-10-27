/-- Charnel Troll -/
card charnelTroll : Spelled :=
  { name := "Charnel Troll", cost := [generic 1, pip .black, pip .green], types := [.creature],
    subtypes := [troll],
    text :=
      [ trample,
        at_ (beginningOfYour .upkeep)
          (doIfDone (exile (a (and [isCreature, in_ yourGraveyard])))
            (putCounters 1 p1p1Counter thisCreature)
            (sacrifice thisCreature)),
        activated
          (compound
            [ [pip .black, pip .green],
              (discard (a (and [isCreature, in_ yourHand]))) ])
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 4, toughness := stat 4 }
