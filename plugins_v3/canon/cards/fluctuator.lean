/-- Fluctuator -/
card fluctuator : Spelled :=
  { name := "Fluctuator", cost := [generic 2], types := [.artifact],
    text :=
      [ static (costShift (allOf (and [abilityHead (.keyword Definitions.cycling),
          did you (activate which) thisObject]))
          (less 2)) ] }
