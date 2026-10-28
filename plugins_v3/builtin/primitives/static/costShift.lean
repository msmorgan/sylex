/-- Authoring primitive for `StaticSpec.costShift`. -/
primitive costShift (subject : NounPhrase) (shift : CostShift) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.costShift subject shift
}
