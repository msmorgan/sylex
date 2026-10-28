card := {
  name := "Silverquill Silencer",
  cost := [white, black],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    entersChoosingFrom thisCreature cardName (nameOfCard (non isLand)),
    whenever (does anOpponent (cast (a [isSpell, named chosenName])))
      [instruct they (loseLife 3), draw 1]
  ],
  power := 3,
  toughness := 2
}
