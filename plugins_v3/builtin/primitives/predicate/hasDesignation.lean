/-- Authoring primitive for `Predicate.hasDesignation`. -/
primitive hasDesignation (designation : DesignationLabel)
    (holder : Option NounPhrase := none) : Predicate := {
  body :=
    Semantics.Primitives.Predicate.hasDesignation designation holder
}
