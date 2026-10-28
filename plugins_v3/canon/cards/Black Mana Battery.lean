/-- Black Mana Battery -/
card := {
  name := "Black Mana Battery",
  cost := [4],
  types := [artifact],
  text := [
    activated (compound [[2], tapSymbol]) (putCounters 1 chargeCounter thisArtifact),
    activated (compound [tapSymbol, (removeCounters anyNumber chargeCounter thisArtifact)])
      [addMana 1 (manaOf [[of black]]), addMana removedThisWay (manaOf [[of black]])]
  ]
}
