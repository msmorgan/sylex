card azusaLostButSeeking : Spelled :=
  { name := "Azusa, Lost but Seeking", cost := [generic 2, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, monk],
    text :=
        [static
           (can you (play (counted (additional (exactly 2)) isLand))
              (timing := onEachOfYourTurns))],
    power := stat 1, toughness := stat 2 }
