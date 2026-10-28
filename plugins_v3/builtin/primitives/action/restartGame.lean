/-- Restart the game. -/
primitive restartGame : Action := {
  spelling := "restart the game",
  body :=
    Semantics.Primitives.Action.restartGame
}
