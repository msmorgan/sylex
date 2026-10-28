card := {
  name := "Distant Melody",
  cost := [3, blue],
  types := [sorcery],
  text := [
    spell
      [
        choose (a (quality (subtype creature))),
        draw (forEach 1 [isPermanent, youControl, ofChosen (subtype creature)])
      ]
  ]
}
