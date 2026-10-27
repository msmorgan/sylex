/-- Rites of Flourishing -/
card ritesOfFlourishing : Spelled :=
  { name := "Rites of Flourishing", cost := [generic 2, pip .green],
    types := [.enchantment],
    text :=
      [ at_ (beginningOfPossessed .drawStep eachPlayer)
          (instruct (that player) (draw 1)),
        static (can eachPlayer (play (counted (additional (exactly 1)) isLand))
          (timing := onEachOfTheirTurns)) ] }
