/-- "<spec> unless <condition>": "<spec> if not <condition>". -/
template onlyUnless (spec : StaticSpec) (condition : Condition) : StaticSpec := {
  spelling := "<spec> unless <condition>",
  body :=
    conditional spec (not condition) Semantics.CondMarking.ifSo
}
