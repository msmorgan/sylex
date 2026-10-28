/-- Adarkar Unicorn -/
card := {
  name := "Adarkar Unicorn",
  cost := [1, white, white],
  types := [creature],
  subtypes := [unicorn],
  text := [
    activated tapSymbol
      (addMana 1 (manaOf [[of blue], [colorless, of blue]])
         [spendOnly [toPay (ofKeyword cumulativeUpkeep)]])
  ],
  power := 2,
  toughness := 2
}
