card := {
  name := "Consuming Tide",
  cost := [2, blue, blue],
  types := [sorcery],
  text := [
    spell
      [
        instruct eachPlayer (choose (a [isPermanent, non isLand, controlledBy they])),
        returnToHand (allOf [isPermanent, non isLand, notChosen]),
        doForEach
          (each
             (compareOver isOpponent (countOf (cardIn (handOf they))) greater
                (countOf (cardIn yourHand))))
          (draw 1)
      ]
  ]
}
