/-- Consuming Tide -/
card consumingTide : Spelled :=
  { name := "Consuming Tide", cost := [generic 2, pip .blue, pip .blue], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ instruct eachPlayer (choose
              (a (and [isPermanent, non isLand, controlledBy they]))),
            returnToHand (allOf (and [isPermanent, non isLand, notChosen])),
            doForEach
              (each (compareOver isOpponent (countOf (cardIn (handOf they))) .greater
                (countOf (cardIn yourHand))))
              (draw 1) ]) ] }
