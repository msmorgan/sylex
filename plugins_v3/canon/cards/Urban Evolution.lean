card := {
  name := "Urban Evolution",
  cost := [3, green, blue],
  types := [sorcery],
  text := [
    spell
      [
        draw 3,
        establishThroughout (can you (play (counted (additional (exactly 1)) isLand))) thisTurn
      ]
  ]
}
