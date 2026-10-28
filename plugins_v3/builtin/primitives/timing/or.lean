/-- At least one timing constraint applies. -/
primitive or (disjuncts : List Timing := []) : Timing := {
  body :=
    Semantics.Primitives.Timing.or disjuncts
}
