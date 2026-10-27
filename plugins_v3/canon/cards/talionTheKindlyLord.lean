/-- Talion, the Kindly Lord -/
card talionTheKindlyLord : Spelled :=
  { name := "Talion, the Kindly Lord", cost := [generic 2, pip .blue, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [faerie, noble],
    text :=
      [ flying,
        static (entersChoosing thisCreature .number),
        whenever
          (does anOpponent (cast (a (and [isSpell,
                      compare .eq chosenNumber (axes := [.stat .manaValue, .stat .power, .stat
                        .toughness])]))))
          (sequentially [instruct (that player) (loseLife 2), draw 1]) ],
    power := stat 3, toughness := stat 4 }
