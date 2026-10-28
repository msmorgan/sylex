/-- Qualify an event by its cause. -/
primitive causes (cause : Causing) (event : GameEvent) : GameEvent := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.GameEvent.causes cause event
}
