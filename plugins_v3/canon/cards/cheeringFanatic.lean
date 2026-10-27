card cheeringFanatic : Spelled :=
  { name := "Cheering Fanatic", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [goblin],
    text :=
      [ whenever (does thisCreature attack)
          (sequentially
            [ choose (a (quality .cardName)),
              establishThroughout (costShift (allOf (and [isSpell, ofChosen .cardName]))
                                     (less 1)) thisTurn ]) ],
    power := stat 2, toughness := stat 2 }
