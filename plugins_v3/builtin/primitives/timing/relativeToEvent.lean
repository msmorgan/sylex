primitive relativeToEvent (side : EventSide) (event : GameEvent) : Timing := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Timing.relativeToEvent side event
}
