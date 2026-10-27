/-- Liliana's Caress -/
card lilianasCaress : Spelled :=
  { name := "Liliana's Caress", cost := [generic 1, pip .black], types := [.enchantment],
    text :=
      [ whenever (does anOpponent (discard (a
        (cardIn hand))))
          (instruct they (loseLife 2)) ] }
