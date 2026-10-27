/-- Ferocious Pup -/
card ferociousPup : Spelled :=
  { name := "Ferocious Pup", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [wolf],
    text :=
      [ when (does thisCreature enter)
          (create 1 (creatureToken 2 2 [.green] [wolf])) ],
    power := stat 0, toughness := stat 1 }
