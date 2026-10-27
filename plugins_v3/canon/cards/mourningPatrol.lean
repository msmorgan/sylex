/-- Mourning Patrol // Morning Apparition -/
card mourningPatrol : Spelled := { body := .transforming
  { characteristics :=
    { name := "Mourning Patrol", cost := [generic 2, pip .white], types := [.creature],
      subtypes := [human, soldier], text := [vigilance, disturb [generic 3, pip .white]],
      power := stat 2, toughness := stat 3 } }
  { characteristics :=
    { name := "Morning Apparition", colors := [.white], types := [.creature],
      subtypes := [spirit, soldier],
      text :=
        [ flying, vigilance,
          static (replace (would any (putInto thisCreature graveyard)) (instead (exile it))) ],
      power := stat 2, toughness := stat 1 } } }
