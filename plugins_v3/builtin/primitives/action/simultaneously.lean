/-- Perform the actions simultaneously. -/
primitive simultaneously (steps : List Action := []) : Action := {
  body :=
    Semantics.Primitives.Action.simultaneously steps
}
