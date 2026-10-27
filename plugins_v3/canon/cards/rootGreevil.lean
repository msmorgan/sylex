card rootGreevil : Spelled :=
  { name := "Root Greevil", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [beast],
    text :=
      [ activated
          (compound [[generic 2, pip .green], tapSymbol, sacrifice thisCreature])
          (destroy (allOf (and [isEnchantment, ofYourChoice .color]))) ],
    power := stat 2, toughness := stat 3 }
