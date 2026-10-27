/-- Sugar Coat -/
card sugarCoat : Spelled :=
  { name := "Sugar Coat", cost := [generic 2, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ flash,
        enchant (or [isCreature, food]),
        static (conjunction [ qualityChange (enchanted permanent) .sets
              (bundle
                { characteristics :=
                  { types := [.artifact], subtypes := [food],
                    text :=
                      [ activated (compound [[generic 2], tapSymbol, (sacrifice
                          thisArtifact)])
                          (gainLife 3) ] } }),
            allAbilityLoss it ]) ] }
