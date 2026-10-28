primitive and (conjuncts : List Timing := []) : Timing := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Timing.and conjuncts
}
