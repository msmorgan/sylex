/-- Culling Mark -/
card cullingMark : Spelled :=
  { name := "Culling Mark", cost := [generic 2, pip .green], types := [.sorcery],
    text :=
      [ spell (establishThroughout (must (target isCreature) block) thisTurn) ] }
