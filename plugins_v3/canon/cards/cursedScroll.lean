card cursedScroll : Spelled :=
  { name := "Cursed Scroll", cost := [generic 1], types := [.artifact],
    text :=
      [ activated (compound [[generic 3], tapSymbol])
          (sequentially
            [ choose (a (quality .cardName)),
              reveal (aAtRandom (cardIn yourHand)),
              doIf (is (that card) (named chosenName))
                (instruct thisArtifact (dealDamage 2 (target isAnyTarget))) ]) ] }
