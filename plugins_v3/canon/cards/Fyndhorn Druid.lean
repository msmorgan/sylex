card := {
  name := "Fyndhorn Druid",
  cost := [2, green],
  types := [creature],
  subtypes := [elf, druid],
  text := [when (does thisCreature die) (if_ := some (was (becomeBlocked it) soFarThisTurn)) (gainLife 4)],
  power := 2,
  toughness := 2
}
