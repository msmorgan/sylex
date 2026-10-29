card := {
  name := "Cultivator Colossus",
  cost := [4, green, green, green],
  types := [creature],
  subtypes := [plant, beast],
  text := [
    trample,
    definesPt thisCreature (countOf [isLand, youControl]),
    when (does thisCreature enter)
      (may (putOntoBattlefieldTapped (a [isLand, in_ yourHand])) [draw 1, repeat_ again])
  ]
}
