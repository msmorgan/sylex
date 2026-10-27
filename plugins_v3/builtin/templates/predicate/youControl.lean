/-- "you control" -/
template youControl : Predicate := {
  spelling := "you control",
  body :=
    Semantics.Templates.Predicate.controlledBy Semantics.Templates.NounPhrase.you
}
