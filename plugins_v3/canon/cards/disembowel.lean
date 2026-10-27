card disembowel : Spelled :=
  { name := "Disembowel", cost := [.variable, pip .black], types := [.instant],
    text :=
      [ spell (destroy (target (and [isCreature, hasManaValue .eq (letter .x)]))) ] }
