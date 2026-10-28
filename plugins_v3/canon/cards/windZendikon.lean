card := {
  name := "Wind Zendikon",
  cost := [blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isLand,
    qualityChange (enchanted land) sets
      (bundle
         {
           colors := [blue],
           types := [creature],
           subtypes := [elemental],
           text := [flying],
           power := 2,
           toughness := 2
         }
         (some land)),
    when (does (enchanted land) die) (returnToHand (that card))
  ]
}
