/-- "until <event>" -/
primitive untilEvent (event : GameEvent) : Duration := {
  spelling := "until <event>",
  body :=
    Semantics.Primitives.Duration.between Option.none (Option.some event)
}
