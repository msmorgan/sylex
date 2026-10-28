/-- Celebrate the Harvest -/
card := {
  name := "Celebrate the Harvest",
  cost := [3, green],
  types := [sorcery],
  text := [
    spell
      [
        search yourLibrary (counted (upToOf (letter x)) [isLand, hasSupertype basic]),
        define x (distinctCount (value power) (allOf [isCreature, youControl])),
        putOntoBattlefieldTapped (those card),
        shuffle
      ]
  ]
}
