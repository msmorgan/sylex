/-- Authoring primitive for `StaticSpec.costShift`. -/
primitive costShift (subject : NounPhrase) (shift : CostShift) : StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.costShift subject shift
}
