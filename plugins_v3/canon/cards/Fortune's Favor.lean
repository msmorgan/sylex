card := {
  name := "Fortune's Favor",
  cost := [3, blue],
  types := [instant],
  text := [
    spell
      [
        instruct (target isOpponent) (lookAt (topSlice 4)),
        instruct they (separateIntoPiles them 2 [faceDown, faceUp]),
        putInto onePile hand,
        putInto theOther graveyard
      ]
  ]
}
