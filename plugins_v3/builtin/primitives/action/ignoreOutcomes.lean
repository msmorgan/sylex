/-- Ignore the specified die-roll outcomes. -/
primitive ignoreOutcomes (which : IgnoredOutcomes) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.ignoreOutcomes which
}
