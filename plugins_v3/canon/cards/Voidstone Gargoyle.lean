card := {
  name := "Voidstone Gargoyle",
  cost := [3, white, white],
  types := [creature],
  subtypes := [gargoyle],
  text := [
    flying,
    entersChoosingFrom thisCreature cardName (nameOfCard (non isLand)),
    cant any (cast (bare [isSpell, named chosenName])),
    cant any
      (activate (allOf [abilityHead anyActivated, abilityOf (allOf [isSource, named chosenName])]))
  ],
  power := 3,
  toughness := 3
}
