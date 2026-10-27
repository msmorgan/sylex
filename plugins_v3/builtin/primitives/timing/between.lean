primitive between (start end_ : GameEvent) : Duration := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Duration.between (Option.some start) (Option.some end_)
}
