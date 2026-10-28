card := {
  name := "Stag Beetle",
  cost := [3, green, green],
  types := [creature],
  subtypes := [insect],
  text := [
    static
      [
        entersWithCounters thisCreature (letter x) p1p1Counter,
        letterDefinition x (countOf [isCreature, isOther])
      ]
  ],
  power := 0,
  toughness := 0
}
