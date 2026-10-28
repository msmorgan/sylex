card := {
  name := "Workhorse",
  cost := [6],
  types := [artifact, creature],
  subtypes := [horse],
  text := [
    entersWithCounters thisCreature 4 p1p1Counter,
    activated (removeCounters (exactly 1) p1p1Counter thisCreature)
      (addMana 1 (manaOf [[colorless]]))
  ],
  power := 0,
  toughness := 0
}
