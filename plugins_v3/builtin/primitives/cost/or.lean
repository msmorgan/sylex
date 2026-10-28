/-- A choice between alternative costs. -/
primitive or (costs : List Cost := []) : Cost := {
  body :=
    Semantics.Primitives.Cost.or costs
}
