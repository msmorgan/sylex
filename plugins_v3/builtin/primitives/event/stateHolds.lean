primitive stateHolds (condition : Condition) : GameEvent := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.GameEvent.stateHolds condition
}
