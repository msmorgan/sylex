card := {
  name := "Cursed Scroll",
  cost := [1],
  types := [artifact],
  text := [
    activated (compound [[3], tapSymbol])
      [
        choose (a (quality cardName)),
        reveal (aAtRandom (cardIn yourHand)),
        doIf (is (that card) (named chosenName))
          (instruct thisArtifact (dealDamage 2 (target isAnyTarget)))
      ]
  ]
}
