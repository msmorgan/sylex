card := {
  name := "Sandstone Oracle",
  cost := [7],
  types := [artifact, creature],
  subtypes := [sphinx],
  text := [
    flying,
    when (does thisCreature enter)
      [
        choose anOpponent,
        doIf
          (compareAmt (countOf (cardIn (handOf (that player)))) greater (countOf (cardIn yourHand)))
          (draw theDifference)
      ]
  ],
  power := 4,
  toughness := 4
}
