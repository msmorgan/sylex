card := {
  name := "History of Benalia",
  cost := [1, white, white],
  types := [enchantment],
  subtypes := [saga],
  text := [
    when (chapterMark [1, 2])
      (create 1
         (written
            {
              colors := [white],
              types := [creature],
              subtypes := [knight],
              text := [vigilance],
              power := 2,
              toughness := 2
            })),
    when (chapterMark [3]) (get (allOf [knight, youControl]) (up 2) (up 1) untilEndOfTurn)
  ]
}
