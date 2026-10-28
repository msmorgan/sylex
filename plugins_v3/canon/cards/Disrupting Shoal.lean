card := {
  name := "Disrupting Shoal",
  cost := [«variable», blue, blue],
  types := [instant],
  subtypes := [arcane],
  text := [
    altCost this_ (exile (a [hasColor blue, hasManaValue eq (letter x), in_ yourHand])),
    spell
      (doOnlyIf (counter (target isSpell)) (compareAmt (statOf (stat manaValue) it) eq (letter x)))
  ]
}
