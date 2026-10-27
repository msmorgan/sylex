/-- Fuming Effigy -/
card fumingEffigy : Spelled :=
  { name := "Fuming Effigy", cost := [generic 3, pip .red], types := [.creature],
    subtypes := [spirit],
    text :=
      [ whenever
          (does (counted (orMore 1) (cardIn yourGraveyard))
             (leave (from_ := yourGraveyard)))
          (instruct this_ (dealDamage 1 eachOpponent)) ],
    power := stat 4, toughness := stat 3 }
