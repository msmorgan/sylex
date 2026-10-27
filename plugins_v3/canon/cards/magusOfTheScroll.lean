card magusOfTheScroll : Spelled :=
  { name := "Magus of the Scroll", cost := [pip .red], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ activated (compound [[generic 3], tapSymbol])
          (sequentially
            [ choose (a (quality .cardName)),
              reveal (aAtRandom (cardIn yourHand)),
              doIf (is (that card) (named chosenName))
                (instruct thisCreature (dealDamage 2 (target isAnyTarget))) ]) ],
    power := stat 1, toughness := stat 1 }
