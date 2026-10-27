card sphinxOfTheChimes : Spelled :=
  { name := "Sphinx of the Chimes", cost := [generic 4, pip .blue, pip .blue], types := [.creature],
    subtypes := [sphinx],
    text :=
      [ flying,
        activated
          (perform
             (discard (withTheSameName (counted (exactly 2) (and [non isLand, in_ yourHand])))))
          (draw 4) ],
    power := stat 5, toughness := stat 6 }
