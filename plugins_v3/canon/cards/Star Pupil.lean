card := {
  name := "Star Pupil",
  cost := [white],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    entersWithCounters thisCreature 1 p1p1Counter,
    when (does thisCreature die)
      (doForEachKind counterKind (it) counterKind
         (putCounters (statOf boundCounter it) thatKind (target [isCreature, youControl])))
  ],
  power := 0,
  toughness := 0
}
