/-- "only once each game" -/
primitive oncePerGame : UsageLimit := {
  spelling := "only once each game",
  body :=
    Semantics.Primitives.UsageLimit.mk 1 Semantics.Period.game
}
