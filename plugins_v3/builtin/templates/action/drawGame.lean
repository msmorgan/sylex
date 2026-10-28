/-- "the game is a draw" [CR#104.4c] -/
-- [CR#104.4c]
template drawGame : Action := {
  spelling := "the game is a draw",
  body := conclude .drawGame
}
