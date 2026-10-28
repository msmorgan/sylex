card := {
  name := "Feral Contest",
  cost := [3, green],
  types := [sorcery],
  text := [
    spell
      [
        putCounters 1 p1p1Counter (target [isCreature, youControl]),
        establishThroughout (must (target [isCreature, isOther]) (block it)) thisTurn
      ]
  ]
}
