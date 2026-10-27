card merfolkSecretkeeper : Spelled := { body := .adventurer
  { characteristics :=
    { name := "Merfolk Secretkeeper", cost := [pip .blue], types := [.creature],
      subtypes := [merfolk, wizard],
      power := stat 0, toughness := stat 4 } }
  { characteristics :=
    { name := "Venture Deeper", cost := [pip .blue], types := [.sorcery],
      subtypes := [adventure],
      text := [ spell (instruct (target isPlayer) (mill 4)) ] } } }
