/-- "the chosen <sort>", revealed -/
primitive choiceOf (sort : ChoiceSort) : Exposed := {
  spelling := "the chosen <sort>",
  body :=
    Semantics.Primitives.Exposed.choice sort
}
