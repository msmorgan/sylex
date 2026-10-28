/-- Authoring primitive for `Predicate.targets`. -/
primitive targeting (subject : NounPhrase) (extent : TargetExtent) : Predicate := {
  body :=
    Semantics.Primitives.Predicate.targets subject extent
}
