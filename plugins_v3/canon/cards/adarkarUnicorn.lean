/-- Adarkar Unicorn -/
card adarkarUnicorn : Spelled :=
  { name := "Adarkar Unicorn", cost := [generic 1, pip .white, pip .white],
    types := [.creature], subtypes := [unicorn],
    text :=
      [ activated tapSymbol
          (addMana 1 (manaOf [[.of .blue], [.colorless, .of .blue]])
            [spendOnly [toPay (.ofKeyword cumulativeUpkeep)]]) ],
    power := stat 2, toughness := stat 2 }
