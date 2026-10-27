/-- Extinction -/
card extinction : Spelled :=
  { name := "Extinction", cost := [generic 4, pip .black], types := [.sorcery],
    text :=
      [ spell (destroy (allOf (and [isCreature, ofYourChoice (.subtype .creature)]))) ] }
