/-- Mutagen Connoisseur -/
card mutagenConnoisseur : Spelled :=
  { name := "Mutagen Connoisseur", cost := [generic 1, pip .green, pip .blue],
    types := [.creature], subtypes := [vedalken, mutant],
    text :=
      [ flying, vigilance,
        static (getsPt thisCreature
          (up (forEach 1 (and [isTransformed, youControl]))) (up 0)) ],
    power := stat 0, toughness := stat 5 }
