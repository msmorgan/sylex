/-- Squelch -/
card squelch : Spelled :=
  { name := "Squelch", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [counter (target (abilityHead .anyActivated)), draw 1]) ]
              }
