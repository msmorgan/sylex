/-- Copy the original spell's or ability's targets to its copy. -/
primitive copyTargets (copy original : NounPhrase) : Action := {
  body :=
    Semantics.Primitives.Action.copyTargets copy original
}
