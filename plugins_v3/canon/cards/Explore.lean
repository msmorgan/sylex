card := {
  name := "Explore",
  cost := [1, green],
  types := [sorcery],
  text := [
    spell
      [
        establishThroughout (can you (play (counted (additional (exactly 1)) isLand))) thisTurn,
        draw 1
      ]
  ]
}
