card := {
  name := "Founding of Omashu",
  cost := [2, red],
  types := [enchantment],
  subtypes := [saga],
  text := [
    when (chapterMark [1]) (create 2 (creatureToken 1 1 [white] [ally])),
    when (chapterMark [2]) (may (discard (a (cardIn yourHand))) (draw 1)),
    when (chapterMark [3]) (get (allOf [isCreature, youControl]) (up 1) (up 0) untilEndOfTurn)
  ]
}
