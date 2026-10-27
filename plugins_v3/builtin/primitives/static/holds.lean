/-- "as though <cond>": the rule applies as though the condition held. -/
primitive holds (cond : Condition) : AsThough := {
  spelling := "as though <cond>",
  body :=
    Semantics.Primitives.AsThough.condition cond
}
