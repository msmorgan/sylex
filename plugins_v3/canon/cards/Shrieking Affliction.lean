card := {
  name := "Shrieking Affliction",
  cost := [black],
  types := [enchantment],
  text := [
    at_ (beginningOfPossessed upkeep eachOpponent)
      (if_ := compareAmt (countOf (cardIn (handOf (that player)))) atMost 1)
      (instruct they (loseLife 3))
  ]
}
