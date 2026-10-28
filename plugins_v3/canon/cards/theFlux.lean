card := {
  name := "The Flux",
  cost := [2, red, red],
  types := [enchantment],
  subtypes := [saga],
  text := [
    when (chapterMark [1]) (instruct this_ (dealDamage 4 (target [isCreature, anOpponentControls]))),
    when (chapterMark [2, 3, 4, 5])
      [exile (topSlice 1), establishThroughout (can you (play (that card))) thisTurn],
    when (chapterMark [6]) (addMana 6 (manaOf [[of red]]))
  ]
}
