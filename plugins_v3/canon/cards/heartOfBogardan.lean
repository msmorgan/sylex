/-- Heart of Bogardan -/
card heartOfBogardan : Spelled :=
  { name := "Heart of Bogardan", cost := [generic 2, pip .red, pip .red], types := [.enchantment],
    text :=
      [ cumulativeUpkeep [generic 2],
        when (does (a isPlayer)
                (pay anyCost .once .unpaid (some thisEnchantment) (some cumulativeUpkeep)))
          (sequentially
            [ simultaneously
                [ instruct this_ (dealDamage (letter .x) (target (or [isPlaneswalker, isPlayer]))),
                  instruct this_ (dealDamage (letter .x) (each (and [isCreature, controlledBy
                    splitOverPlaneswalker]))) ],
              define .x
                (minus (times 2 (countersOn ageCounter thisEnchantment))
                   2) ]) ] }
