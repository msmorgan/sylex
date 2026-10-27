/-- Germination Practicum -/
card germinationPracticum : Spelled :=
  { name := "Germination Practicum", cost := [generic 3, pip .green, pip .green],
    types := [.sorcery], subtypes := [lesson],
    text :=
      [ spell (putCounters 2 p1p1Counter (each (and [isCreature, youControl]))),
        paradigm ] }
