/-- All of the listed conditions hold. -/
primitive and (conjuncts : List Condition := []) : Condition := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Condition.and conjuncts
}
