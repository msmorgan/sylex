/-- Crimson Wisps -/
card crimsonWisps : Spelled :=
  { name := "Crimson Wisps", cost := [pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ establishFor (target isCreature)
              [qualityChange it .sets (colored (.some [.red])), abilityGrant it haste]
              untilEndOfTurn,
            draw 1 ]) ] }
