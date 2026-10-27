/-- "<spec> as long as <condition>" -/
primitive onlyWhile (spec : StaticSpec) (condition : Condition) : StaticSpec := {
  spelling := "<spec> as long as <condition>",
  body :=
    Semantics.Primitives.StaticSpec.conditional spec condition Semantics.CondMarking.asLongAs
}
