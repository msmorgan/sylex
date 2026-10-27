/-- "while <event>" -/
primitive whileDoing (event : GameEvent) : Timing := {
  spelling := "while <event>",
  body :=
    Semantics.Primitives.Timing.whileDoing event
}
