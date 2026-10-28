/-- Ignore the specified die-roll outcomes. -/
primitive ignoreOutcomes (which : IgnoredOutcomes) : Action := {
  body :=
    Semantics.Primitives.Action.ignoreOutcomes which
}
