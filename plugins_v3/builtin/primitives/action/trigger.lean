/-- "<ability> triggers": the act a trigger prohibition names. -/
primitive trigger : Action := {
  spelling := "triggers",
  body :=
    Semantics.Primitives.Action.trigger
}
