/-- Smite -/
card smite : Spelled :=
  { name := "Smite", cost := [pip .white], types := [.instant],
    text := [spell (destroy (target (and [isCreature, isBlocked])))] }
