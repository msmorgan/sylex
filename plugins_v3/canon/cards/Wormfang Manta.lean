card := {
  name := "Wormfang Manta",
  cost := [5, blue, blue],
  types := [creature],
  subtypes := [nightmare, fish, beast],
  text := [
    flying,
    when (does thisCreature enter) (skipPart turn 1),
    when (does thisCreature leave) (extraTurn 1)
  ],
  power := 6,
  toughness := 1
}
