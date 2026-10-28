card := {
  name := "Sunscour",
  cost := [5, white, white],
  types := [sorcery],
  text := [
    altCost this_ (exile (counted (exactly 2) [hasColor white, in_ yourHand])),
    spell (destroy (allOf isCreature))
  ]
}
