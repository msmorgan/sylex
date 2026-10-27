primitive durationBetween (start : Option Semantics.GameEvent) (end_ : Option Semantics.GameEvent) : Semantics.Duration := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Duration.between start end_
}
