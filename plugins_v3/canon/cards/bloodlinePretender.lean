/-- Bloodline Pretender -/
card bloodlinePretender : Spelled :=
  { name := "Bloodline Pretender", cost := [generic 3], types := [.artifact, .creature],
    subtypes := [shapeshifter],
    text :=
      [ changeling,
        static (entersChoosing thisCreature (.subtype .creature)),
        whenever
          (does (a (and [ isCreature, youControl, isOther,
                              ofChosen (.subtype .creature) ])) enter)
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 2, toughness := stat 2 }
