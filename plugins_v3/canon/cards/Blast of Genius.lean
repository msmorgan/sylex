card := {
  name := "Blast of Genius",
  cost := [4, blue, red],
  types := [sorcery],
  text := [
    spell
      [
        choose (target isAnyTarget),
        draw 3,
        discard (a (cardIn yourHand)),
        instruct this_
          (dealDamage (statOf (stat manaValue) (the [was (discard which), isCard])) thatJoin)
      ]
  ]
}
