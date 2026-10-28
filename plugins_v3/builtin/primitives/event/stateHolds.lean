/-- An event described by a condition holding. -/
primitive stateHolds (condition : Condition) : GameEvent := {
  body :=
    Semantics.Primitives.GameEvent.stateHolds condition
}
