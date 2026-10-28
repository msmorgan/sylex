card := {
  name := "Earthen Goo",
  cost := [2, red],
  types := [creature],
  subtypes := [ooze],
  text := [
    trample,
    cumulativeUpkeep (either [red] [green]),
    getsPt thisCreature (up (times 1 (countersOn ageCounter it)))
      (up (times 1 (countersOn ageCounter it)))
  ],
  power := 2,
  toughness := 2
}
