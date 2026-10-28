card := {
  name := "Daydream",
  cost := [white],
  types := [sorcery],
  text := [
    spell
      [
        exile (target [isCreature, youControl]),
        returnToBattlefieldWithCounters (that card) (ownerOf (that card)) 1 p1p1Counter
      ],
    flashback [2, white]
  ]
}
