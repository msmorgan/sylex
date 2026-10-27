/-- Sandstone Oracle -/
card sandstoneOracle : Spelled :=
  { name := "Sandstone Oracle", cost := [generic 7], types := [.artifact, .creature],
    subtypes := [sphinx],
    text :=
      [ flying,
        when (does thisCreature enter)
          (sequentially
            [ choose anOpponent,
              doIf (compareAmt (countOf (cardIn (handOf (that player)))) .greater
                      (countOf (cardIn yourHand)))
                (draw theDifference) ]) ],
    power := stat 4, toughness := stat 4 }
