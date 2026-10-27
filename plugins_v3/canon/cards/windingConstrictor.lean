/-- Winding Constrictor -/
card windingConstrictor : Spelled :=
  { name := "Winding Constrictor", cost := [pip .black, pip .green], types := [.creature],
    subtypes := [snake],
    text :=
      [ static
          (replace
             (would any
                (putCounters anyAmount anyKind (a (and [or [isArtifact, isCreature], youControl]))))
             (instead (putCounters (plus thatMuch 1) thoseCounters
                                    (that permanent)))),
        static (replace (would any (putCounters anyAmount anyKind you)) (instead (putCounters (plus thatMuch 1) thoseCounters you))) ],
    power := stat 2, toughness := stat 3 }
