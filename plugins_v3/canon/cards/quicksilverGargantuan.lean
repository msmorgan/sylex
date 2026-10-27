/-- Quicksilver Gargantuan -/
card quicksilverGargantuan : Spelled :=
  { name := "Quicksilver Gargantuan", cost := [generic 5, pip .blue, pip .blue],
    types := [.creature], subtypes := [shapeshifter],
    text :=
      [ static (enters thisCreature
          [asCopyOf true (a isCreature) [hasPT 7 7]]) ],
    power := stat 7, toughness := stat 7 }
