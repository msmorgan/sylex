/-- "<spec> if <condition>" -/
primitive onlyIfSo (spec : StaticSpec) (condition : Condition) : StaticSpec := {
  spelling := "<spec> if <condition>",
  body :=
    Semantics.Primitives.StaticSpec.conditional spec condition Semantics.CondMarking.ifSo
}
