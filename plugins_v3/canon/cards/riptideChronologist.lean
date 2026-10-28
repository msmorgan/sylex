card := {
  name := "Riptide Chronologist",
  cost := [3, blue, blue],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    activated (compound [[blue], sacrifice thisCreature])
      (untap (allOf [isCreature, ofYourChoice (subtype creature)]))
  ],
  power := 1,
  toughness := 3
}
