card newPerspectives : Spelled :=
  { name := "New Perspectives", cost := [generic 5, pip .blue], types := [.enchantment],
    text :=
      [ when (does thisEnchantment enter) (draw 3),
        (static (onlyWhile (altCost (allOf (abilityHead (.keyword cycling))) [generic 0])
              (compareAmt (countOf (cardIn yourHand)) .atLeast 7))) ] }
