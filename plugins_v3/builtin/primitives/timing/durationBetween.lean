primitive durationBetween (start : Option GameEvent) (end_ : Option GameEvent) : Duration := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Duration.between start end_
}
