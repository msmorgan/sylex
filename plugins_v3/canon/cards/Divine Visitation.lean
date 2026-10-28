card := {
  name := "Divine Visitation",
  cost := [3, white, white],
  types := [enchantment],
  text := [
    replace
      (would any
         (createObject anyAmount (token (copyOf (counted (orMore 1) [isCreature, isToken])) []) you))
      (instead
         (create groupSize
            (written
               {
                 colors := [white],
                 types := [creature],
                 subtypes := [angel],
                 text := [flying, vigilance],
                 power := 4,
                 toughness := 4
               })))
  ]
}
