/-- A duration bounded by optional starting and ending events. -/
primitive durationBetween (start : Option GameEvent) (end_ : Option GameEvent) : Duration := {
  body := Semantics.Primitives.Duration.between start end_
}
