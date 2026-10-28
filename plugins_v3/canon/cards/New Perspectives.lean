card := {
  name := "New Perspectives",
  cost := [5, blue],
  types := [enchantment],
  text := [
    when (does thisEnchantment enter) (draw 3),
    onlyWhile (altCost (allOf (abilityHead (keyword cycling))) [0])
      (compareAmt (countOf (cardIn yourHand)) atLeast 7)
  ]
}
