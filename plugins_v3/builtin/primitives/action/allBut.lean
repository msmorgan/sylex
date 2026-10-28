/-- "ignore all but the <extreme> roll" -/
primitive allBut (extreme : RollExtreme) : IgnoredOutcomes := {
  spelling := "ignore all but the <extreme> roll",
  body :=
    Semantics.Primitives.IgnoredOutcomes.allBut extreme
}
