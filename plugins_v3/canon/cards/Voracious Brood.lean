card := {
  name := "Voracious Brood",
  cost := [2, green],
  types := [creature],
  subtypes := [alien, insect],
  text := [
    entersWithCounters thisCreature (forEach 1 [isCreature, in_ yourGraveyard]) p1p1Counter,
    whenever
      (does any
         (putInto (counted (orMore 1) [isCard, isCreature]) yourGraveyard (from_ := some any)))
      (putCounters groupSize p1p1Counter thisCreature)
  ],
  power := 1,
  toughness := 1
}
