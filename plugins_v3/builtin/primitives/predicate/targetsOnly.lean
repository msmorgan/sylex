/-- "that targets only <subject>" [CR#115.9c] -/
-- [CR#115.9c]
primitive targetsOnly (subject : NounPhrase) : Predicate := {
  spelling := "that targets only <subject>",
  body :=
    Semantics.Primitives.Predicate.targets subject Semantics.TargetExtent.only
}
