card := {
  name := "Cheering Fanatic",
  cost := [1, red],
  types := [creature],
  subtypes := [goblin],
  text := [
    whenever (does thisCreature attack)
      [
        choose (a (quality cardName)),
        establishThroughout (costShift (allOf [isSpell, ofChosen cardName]) (less 1)) thisTurn
      ]
  ],
  power := 2,
  toughness := 2
}
