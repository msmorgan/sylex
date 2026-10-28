/-- Authoring primitive for `StaticSpec.conditional`. -/
primitive conditional (spec : StaticSpec) (condition : Condition) (marking : CondMarking) :
    StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.conditional spec condition marking
}
