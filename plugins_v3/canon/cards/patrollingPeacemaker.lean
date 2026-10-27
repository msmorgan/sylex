/-- Patrolling Peacemaker -/
card patrollingPeacemaker : Spelled :=
  { name := "Patrolling Peacemaker", cost := [generic 2, pip .white],
    types := [.artifact, .creature], subtypes := [robot, soldier],
    text :=
      [ static (entersWithCounters thisCreature 2 p1p1Counter),
        whenever (does anOpponent commitCrime) proliferate ],
    power := stat 0, toughness := stat 0 }
