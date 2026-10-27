primitive actionDelay (event : Semantics.GameEvent) (alternatives : List Semantics.GameEvent) (duration : Option Semantics.Duration) (body : Semantics.Action) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.delay event alternatives duration body
}
