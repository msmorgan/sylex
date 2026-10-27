/-- Agate-Blade Assassin -/
card agateBladeAssassin : Spelled :=
  { name := "Agate-Blade Assassin", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [lizard, assassin],
    text :=
      [ whenever (does thisCreature attack)
          (sequentially
            [instruct defendingPlayer (loseLife 1), gainLife 1]) ],
    power := stat 1, toughness := stat 3 }
