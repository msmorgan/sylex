/-- Copy the original spell's or ability's targets to its copy. -/
primitive copyTargets (copy original : NounPhrase) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.copyTargets copy original
}
