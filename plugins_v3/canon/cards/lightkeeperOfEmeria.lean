/-- Lightkeeper of Emeria -/
card lightkeeperOfEmeria : Spelled :=
  { name := "Lightkeeper of Emeria", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [angel],
    text :=
      [ multikicker [pip .white],
        flying,
        when (does thisCreature enter)
          (gainLife (times 2 (timesPaid (.byKeyword kicker) thisCreature))) ],
    power := stat 2, toughness := stat 4 }
