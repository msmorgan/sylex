/-- Unwinding Clock -/
card := {
  name := "Unwinding Clock",
  cost := [4],
  types := [artifact],
  text := [untapDuring (allOf [isArtifact, youControl]) (each isOtherPlayer)]
}
