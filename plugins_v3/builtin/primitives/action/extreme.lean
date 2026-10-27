/-- "ignore the <extreme> roll" -/
primitive extreme (extreme : RollExtreme) : IgnoredOutcomes := {
  spelling := "ignore the <extreme> roll",
  body :=
    Semantics.Primitives.IgnoredOutcomes.extreme extreme
}
