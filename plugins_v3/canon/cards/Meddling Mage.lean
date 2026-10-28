card := {
  name := "Meddling Mage",
  cost := [white, blue],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    entersChoosingFrom thisCreature cardName (nameOfCard (non isLand)),
    cant any (cast (bare [isSpell, named chosenName]))
  ],
  power := 2,
  toughness := 2
}
