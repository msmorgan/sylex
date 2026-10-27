/-- Balemurk Leech -/
card balemurkLeech : Spelled :=
  { name := "Balemurk Leech", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [leech],
    text :=
      [ eerie
          (whenever
            (does (a (and [isEnchantment, youControl])) enter)
            (joins := [ joinedHead
                (does you (fullyUnlock (a room)))
                    ])
            (instruct eachOpponent (loseLife 1))) ],
    power := stat 2, toughness := stat 2 }
