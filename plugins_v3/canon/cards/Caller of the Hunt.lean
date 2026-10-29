card := {
  name := "Caller of the Hunt",
  cost := [2, green],
  types := [creature],
  subtypes := [human],
  text := [
    addedCost (choose (a (quality (subtype creature)))) false,
    definesPt thisCreature (countOf [isCreature, ofChosen (subtype creature)])
  ]
}
