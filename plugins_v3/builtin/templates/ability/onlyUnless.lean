/-- "<spec> unless <condition>": "<spec> if not <condition>". -/
template onlyUnless (spec : StaticSpec) (condition : Condition) : StaticSpec := {
  spelling := "<spec> unless <condition>",
  body :=
    Semantics.Templates.StaticSpec.conditional spec (Semantics.Templates.Condition.not condition) Semantics.CondMarking.ifSo
}
