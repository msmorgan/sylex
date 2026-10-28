card := {
  name := "Phyrexian Revoker",
  cost := [2],
  types := [artifact, creature],
  subtypes := [phyrexian, horror],
  text := [
    entersChoosingFrom thisCreature cardName (nameOfCard (non isLand)),
    cant any
      (activate (allOf [abilityHead anyActivated, abilityOf (allOf [isSource, named chosenName])]))
  ],
  power := 2,
  toughness := 1
}
