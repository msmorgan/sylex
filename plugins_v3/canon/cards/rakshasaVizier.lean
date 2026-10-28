card := {
  name := "Rakshasa Vizier",
  cost := [2, black, green, blue],
  types := [creature],
  subtypes := [demon],
  text := [
    whenever
      (does any
         (putInto (counted (orMore 1) (cardIn yourGraveyard)) exileZone (from_ := yourGraveyard)))
      (putCounters groupSize p1p1Counter thisCreature)
  ],
  power := 4,
  toughness := 4
}
