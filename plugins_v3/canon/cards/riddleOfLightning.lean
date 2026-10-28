card := {
  name := "Riddle of Lightning",
  cost := [3, red, red],
  types := [instant],
  text := [
    spell
      [
        choose (target isAnyTarget),
        scry 3,
        reveal (topSlice 1),
        instruct this_ (dealDamage (statOf (stat manaValue) (that card)) thatJoin)
      ]
  ]
}
