/-- Perform the actions in order. -/
primitive sequentially (steps : List Action := []) : Action := {
  body :=
    Semantics.Primitives.Action.sequentially steps
}
