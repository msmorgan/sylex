card := {
  name := "Matopi Golem",
  cost := [5],
  types := [artifact, creature],
  subtypes := [golem],
  text := [
    activated [1]
      (triggerThisWay (regenerate thisCreature) (does any (regeneration thisCreature))
         (putCounters 1 m1m1Counter it))
  ],
  power := 3,
  toughness := 3
}
