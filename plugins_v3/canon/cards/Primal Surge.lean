card := {
  name := "Primal Surge",
  cost := [8, green, green],
  types := [sorcery],
  text := [
    spell
      [
        exile (topSlice 1),
        doIf (is it isPermanentCard) (may (putOntoBattlefield it) (repeat_ again))
      ]
  ]
}
