/-- "any cost" -/
primitive anyCost : Cost := {
  spelling := "any cost",
  body :=
    Semantics.Primitives.Cost.anyCost
}
