card := {
  name := "Natural Affinity",
  cost := [2, green],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (qualityChange (allOf isLand) sets
            (bundle { types := [creature], power := 2, toughness := 2 } (some land)))
         untilEndOfTurn)
  ]
}
