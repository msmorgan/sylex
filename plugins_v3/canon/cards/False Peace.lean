card := {
  name := "False Peace",
  cost := [white],
  types := [sorcery],
  text := [spell (establishThroughout (partSkip (target isPlayer) combat) (duringNextTurnOf (that player)))]
}
