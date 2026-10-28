/-- All of the timing constraints apply. -/
primitive and (conjuncts : List Timing := []) : Timing := {
  body :=
    Semantics.Primitives.Timing.and conjuncts
}
