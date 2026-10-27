/-- "the game is a draw" [CR#104.4c] -/
-- [CR#104.4c]
primitive drawGame : Action := {
  spelling := "the game is a draw",
  body :=
    Semantics.Primitives.Action.conclude Semantics.OutcomeVerb.drawGame
}
