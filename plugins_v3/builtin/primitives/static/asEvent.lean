primitive asEvent (event : GameEvent) (does : Action) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.asEvent event does
}
