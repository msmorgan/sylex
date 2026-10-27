card shuFarmer : Spelled :=
  { name := "Shu Farmer", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [human],
    text :=
      [ activatedOnlyDuring tapSymbol (gainLife 1) (beforePart
          .declareAttackers you) ],
    power := stat 1, toughness := stat 1 }
