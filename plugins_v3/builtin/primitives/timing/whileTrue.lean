/-- "while <condition>" -/
primitive whileTrue (condition : Condition) : Timing := {
  spelling := "while <condition>",
  body :=
    Semantics.Primitives.Timing.onlyIf condition
}
