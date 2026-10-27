primitive timingRelativeToEvent (side : Semantics.EventSide) (event : Semantics.GameEvent) : Semantics.Timing := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Timing.relativeToEvent side event
}
