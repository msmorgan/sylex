/-- Festival -/
card festival : Spelled :=
  { name := "Festival", cost := [pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout (cant (allOf isCreature) attack) thisTurn)
          (timing := duringPart .upkeep anOpponent) ] }
