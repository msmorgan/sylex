/-- "until <event>" -/
template untilEvent (event : GameEvent) : Duration := {
  spelling := "until <event>",
  body := durationBetween none (some event)
}
