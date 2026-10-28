/-- A cost paid by performing an action. -/
primitive perform (action : Action) : Cost := {
  body :=
    Semantics.Primitives.Cost.perform action
}
