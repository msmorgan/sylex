/-- Dark Depths -/
card darkDepths : Spelled :=
  { name := "Dark Depths", supertypes := [.legendary, .snow], types := [.land],
    text :=
      [ static (entersWithCounters thisLand 10 iceCounter),
        activated [generic 3]
          (removeCounters (exactly 1) iceCounter thisLand),
        when (stateHolds (not (is thisLand (hasCounter iceCounter))))
          (doIfDone (sacrifice thisLand) (create 1 maritLage))
              ] }
