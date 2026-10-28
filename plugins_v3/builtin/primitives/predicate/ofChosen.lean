/-- "of the chosen <quality>" -/
primitive ofChosen (sort : QualitySort) (ref : ChoiceRef := .theChoice) : Predicate := {
  body := Semantics.Primitives.Predicate.ofChosen ref sort
}
