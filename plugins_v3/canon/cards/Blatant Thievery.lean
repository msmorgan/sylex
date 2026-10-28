card := {
  name := "Blatant Thievery",
  cost := [4, blue, blue, blue],
  types := [sorcery],
  text := [spell (doForEach eachOpponent (gainControl (target (controlledBy (that player)))))]
}
