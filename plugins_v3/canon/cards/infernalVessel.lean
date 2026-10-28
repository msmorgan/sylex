card := {
  name := "Infernal Vessel",
  cost := [2, black],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    when (does thisCreature die) (if_ := some (not (is it demon)))
      [
        returnToBattlefieldWithCounters it (ownerOf it) 2 p1p1Counter,
        become it { subtypes := [demon] }
      ]
  ],
  power := 2,
  toughness := 1
}
