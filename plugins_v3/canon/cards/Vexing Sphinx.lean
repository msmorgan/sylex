card := {
  name := "Vexing Sphinx",
  cost := [1, blue, blue],
  types := [creature],
  subtypes := [sphinx],
  text := [
    flying,
    cumulativeUpkeep (discard (a (cardIn yourHand))),
    when (does thisCreature die) (draw (countersOn ageCounter it))
  ],
  power := 4,
  toughness := 4
}
