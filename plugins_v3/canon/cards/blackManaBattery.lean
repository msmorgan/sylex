/-- Black Mana Battery -/
card blackManaBattery : Spelled :=
  { name := "Black Mana Battery", cost := [generic 4], types := [.artifact],
    text :=
      [ activated (compound [[generic 2], tapSymbol])
          (putCounters 1 chargeCounter thisArtifact),
        activated
          (compound [tapSymbol,
            (removeCounters anyNumber chargeCounter thisArtifact)])
          (sequentially
            [ addMana 1 (manaOf [[.of .black]]),
              addMana removedThisWay (manaOf [[.of .black]]) ]) ] }
