/-- Authoring primitive for `StaticSpec.conditional`. -/
primitive conditional (spec : StaticSpec) (condition : Condition) (marking : CondMarking) :
    StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.conditional spec condition marking
}
