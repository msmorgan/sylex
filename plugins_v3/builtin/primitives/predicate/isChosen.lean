/-- "chosen": one the choice in scope chose ("the chosen creature", "the chosen cards"). -/
primitive isChosen : Predicate := {
  spelling := "chosen",
  body :=
    Semantics.Primitives.Predicate.chosen Semantics.ChoiceRef.theChoice
}
