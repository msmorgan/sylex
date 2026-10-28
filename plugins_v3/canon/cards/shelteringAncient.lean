card := {
  name := "Sheltering Ancient",
  cost := [1, green],
  types := [creature],
  subtypes := [treefolk],
  text := [trample, cumulativeUpkeep (putCounters 1 p1p1Counter (a [isCreature, anOpponentControls]))],
  power := 5,
  toughness := 5
}
