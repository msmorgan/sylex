card := {
  name := "Sage of Fables",
  cost := [2, blue],
  types := [creature],
  subtypes := [merfolk, wizard],
  text := [
    entersWithAdditionalCounters (each [isCreature, wizard, youControl, isOther]) 1 p1p1Counter,
    activated
      (compound [[2], (removeCounters (exactly 1) p1p1Counter (a [isCreature, youControl]))])
      (draw 1)
  ],
  power := 2,
  toughness := 2
}
