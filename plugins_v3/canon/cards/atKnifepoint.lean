card := {
  name := "At Knifepoint",
  cost := [1, black, red],
  types := [enchantment],
  text := [
    partScope turn you (abilityGrant (allOf [isOutlaw, youControl]) firstStrike),
    (whenever (does you commitCrime)
       (create 1
          (written
             {
               colors := [red],
               types := [creature],
               subtypes := [mercenary],
               text := [
                 activatedOnlyDuring tapSymbol
                   (get (target [isCreature, youControl]) (up 1) (up 0) untilEndOfTurn) asSorcery
               ],
               power := 1,
               toughness := 1
             }))
       (limit := oncePerTurn))
  ]
}
