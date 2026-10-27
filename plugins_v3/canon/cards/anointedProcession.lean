card anointedProcession : Spelled :=
  { name := "Anointed Procession", cost := [generic 3, pip .white], types := [.enchantment],
    text :=
      [ static (replace (causedBy byAnEffect (would any (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) []) you))) (instead (create (times 2 groupSize) asThose []))) ] }
