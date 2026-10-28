card := {
  name := "Mistform Dreamer",
  cost := [2, blue],
  types := [creature],
  subtypes := [illusion],
  text := [
    flying,
    activated [1]
      (establishThroughout
         (qualityChange thisCreature sets (chosenQuality (ofYourChoice (subtype creature))))
         untilEndOfTurn)
  ],
  power := 2,
  toughness := 1
}
