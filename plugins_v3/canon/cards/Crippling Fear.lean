card := {
  name := "Crippling Fear",
  cost := [2, black, black],
  types := [sorcery],
  text := [
    spell
      [
        choose (a (quality (subtype creature))),
        get (allOf [isCreature, not (ofChosen (subtype creature))]) (down 3) (down 3) untilEndOfTurn
      ]
  ]
}
