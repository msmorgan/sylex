/-- Riddles in the Dark -/
card := {
  name := "Riddles in the Dark",
  cost := [2, blue],
  types := [instant],
  text := [
    spell
      [
        lookAt (topSlice 4),
        separateIntoPiles them 2 [faceDown, faceUp],
        instruct anOpponent (choose onePile),
        putInto (that pile) hand,
        putInto theOther graveyard
      ]
  ]
}
