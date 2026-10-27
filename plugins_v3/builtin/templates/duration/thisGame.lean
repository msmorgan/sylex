/-- "this game" -/
template thisGame : Duration := {
  spelling := "this game",
  body :=
    Semantics.Templates.Duration.durationBetween Option.none (Option.some Semantics.Templates.GameEvent.gameEventNow)
}
