/-- Authoring primitive for `Predicate.notChosen`. -/
primitive notChosen : Predicate := {
  spelling := "not chosen",
  body :=
    Semantics.Primitives.Predicate.notChosen
}
