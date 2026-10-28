/-- Change the actor's life total by the given Delta. -/
primitive changeLife (delta : Delta Amount) : Action := {
  body :=
    Semantics.Primitives.Action.changeLife delta
}
