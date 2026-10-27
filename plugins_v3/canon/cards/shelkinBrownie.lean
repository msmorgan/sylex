/-- Shelkin Brownie -/
card shelkinBrownie : Spelled :=
  { name := "Shelkin Brownie", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [ouphe],
    text :=
      [ activated (compound [tapSymbol])
          (establishThroughout
             (abilityLoss (target isCreature)
                [abilities bandsWithOtherAbilities]) untilEndOfTurn) ],
    power := stat 1, toughness := stat 1 }
