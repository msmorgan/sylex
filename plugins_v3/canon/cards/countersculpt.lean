/-- Countersculpt -/
card countersculpt : Spelled :=
  { name := "Countersculpt", cost := [pip .blue, pip .blue], types := [.instant],
    text :=
      [ static (addedCost (or [behold jace, [generic 1]]) false),
        spell (sequentially [counter (target isSpell), empowerJace 1]) ] }
