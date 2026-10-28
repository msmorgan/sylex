card := {
  name := "Patrolling Peacemaker",
  cost := [2, white],
  types := [artifact, creature],
  subtypes := [robot, soldier],
  text := [
    entersWithCounters thisCreature 2 p1p1Counter,
    whenever (does anOpponent commitCrime) proliferate
  ],
  power := 0,
  toughness := 0
}
