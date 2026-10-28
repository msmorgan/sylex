/-- Put a Static Spec in force for the given duration. -/
primitive establish (spec : StaticSpec) (duration : Option Duration := none) : Action := {
  body :=
    Semantics.Primitives.Action.establish spec duration
}
