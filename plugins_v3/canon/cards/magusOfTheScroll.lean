card := {
  name := "Magus of the Scroll",
  cost := [red],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    activated (compound [[3], tapSymbol])
      [
        choose (a (quality cardName)),
        reveal (aAtRandom (cardIn yourHand)),
        doIf (is (that card) (named chosenName))
          (instruct thisCreature (dealDamage 2 (target isAnyTarget)))
      ]
  ],
  power := 1,
  toughness := 1
}
