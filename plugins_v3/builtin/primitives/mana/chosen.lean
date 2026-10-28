/-- Full-arity binding for `ColorTerm.chosen`. -/
primitive chosen (ref : ChoiceRef := .theChoice) : ColorTerm := {
  body := Semantics.Primitives.ColorTerm.chosen ref
}
