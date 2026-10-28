card := {
  name := "Bifurcate",
  cost := [3, green],
  types := [sorcery],
  text := [
    spell
      [
        search yourLibrary
          (a [isPermanentCard, (shares cardName share (target [isCreature, isNontoken]))]),
        putOntoBattlefield (that card),
        shuffle
      ]
  ]
}
