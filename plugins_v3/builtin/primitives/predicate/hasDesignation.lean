/-- Authoring primitive for `Predicate.hasDesignation`. -/
primitive hasDesignation (designation : DesignationLabel)
    (holder : Option NounPhrase := none) : Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.hasDesignation designation holder
}
