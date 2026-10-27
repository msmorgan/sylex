card deathsShadow : Spelled :=
  { name := "Death's Shadow", cost := [pip .black], types := [.creature],
    subtypes := [avatar],
    text :=
      [ static (conjunction [ getsPt thisCreature (down (letter .x)) (down (letter .x)),
            letterDefinition .x (lifeTotalOf you) ]) ],
    power := stat 13, toughness := stat 13 }
