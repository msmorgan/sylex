card := {
  name := "Answered Prayers",
  cost := [1, white, white],
  types := [enchantment],
  text := [
    when (does (a [isCreature, youControl]) enter)
      [
        gainLife 1,
        doIf (not (is thisEnchantment isCreature))
          (become thisEnchantment
             {
               types := [creature],
               subtypes := [angel],
               text := [flying],
               power := 3,
               toughness := 3
             }
             untilEndOfTurn)
      ]
  ]
}
