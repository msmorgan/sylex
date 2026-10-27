/-- Stifle -/
card stifle : Spelled :=
  { name := "Stifle", cost := [pip .blue], types := [.instant],
    text :=
      [ spell (counter (target (or [abilityHead .anyActivated, abilityHead .anyTriggered]))) ] }
