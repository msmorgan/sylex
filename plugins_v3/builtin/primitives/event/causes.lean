/-- Qualify an event by its cause. -/
primitive causes (cause : Causing) (event : GameEvent) : GameEvent := {
  body :=
    Semantics.Primitives.GameEvent.causes cause event
}
