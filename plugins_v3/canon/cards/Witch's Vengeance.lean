/-- Witch's Vengeance -/
card := {
  name := "Witch's Vengeance",
  cost := [1, black, black],
  types := [sorcery],
  text := [
    spell
      (get (allOf [isCreature, ofYourChoice (subtype creature)]) (down 3) (down 3) untilEndOfTurn)
  ]
}
