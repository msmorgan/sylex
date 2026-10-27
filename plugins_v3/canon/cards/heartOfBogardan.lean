/-- Heart of Bogardan -/
card heartOfBogardan : Spelled :=
  { name := "Heart of Bogardan", cost := [generic 2, pip .red, pip .red], types := [.enchantment],
    text :=
      [ cumulativeUpkeep [generic 2],
        when heartOfBogardanHeader
          (sequentially
            [ simultaneously
                [ instruct this_ (dealDamage (letter .x) targetPlayerOrPlaneswalker),
                  instruct this_ (dealDamage (letter .x) eachCreatureThatSplitControls) ],
              define .x
                (minus (times 2 (countersOn ageCounter thisEnchantment))
                   2) ]) ] }
