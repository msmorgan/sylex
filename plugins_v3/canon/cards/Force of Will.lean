card := {
  name := "Force of Will",
  cost := [3, blue, blue],
  types := [instant],
  text := [
    altCost this_ (compound [payLife 1, (exile (a [hasColor blue, in_ yourHand]))]),
    spell (counter (target isSpell))
  ]
}
