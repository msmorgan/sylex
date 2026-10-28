/-- Authoring primitive for `Predicate.targets`. -/
primitive targeting (subject : NounPhrase) (extent : TargetExtent) : Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.targets subject extent
}
