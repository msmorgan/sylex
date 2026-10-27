/-- Reciprocate -/
card reciprocate : Spelled :=
  { name := "Reciprocate", cost := [pip .white], types := [.instant],
    text :=
      [ spell (exile
          (target
             (and
                [isCreature, did which (dealDamage anyAmount you) (within soFarThisTurn)]))) ] }
