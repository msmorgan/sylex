card := {
  name := "Coretapper",
  cost := [2],
  types := [artifact, creature],
  subtypes := [myr],
  text := [
    activated tapSymbol (putCounters 1 chargeCounter (target isArtifact)),
    activated (sacrifice thisCreature) (putCounters 2 chargeCounter (target isArtifact))
  ],
  power := 1,
  toughness := 1
}
