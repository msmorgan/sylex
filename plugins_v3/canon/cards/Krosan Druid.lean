card := {
  name := "Krosan Druid",
  cost := [2, green],
  types := [creature],
  subtypes := [centaur, druid],
  text := [
    kicker [4, green],
    when (does thisCreature enter) (if_ := keywordCostPaid kicker thisCreature) (gainLife 10)
  ],
  power := 2,
  toughness := 3
}
