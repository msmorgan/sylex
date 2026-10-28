/-- A cost composed of all the listed costs. -/
primitive compound (costs : List Cost := []) : Cost := {
  body :=
    Semantics.Primitives.Cost.compound costs
}
