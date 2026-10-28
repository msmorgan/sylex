primitive or (disjuncts : List Timing := []) : Timing := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Timing.or disjuncts
}
