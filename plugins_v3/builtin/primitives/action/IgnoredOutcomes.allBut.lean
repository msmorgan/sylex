/-- "ignore all but the <extreme> roll" -/
primitive IgnoredOutcomes.allBut (extreme : RollExtreme) : IgnoredOutcomes := {
  spelling := "ignore all but the <extreme> roll",
  body :=
    Semantics.Primitives.IgnoredOutcomes.allBut extreme
}
