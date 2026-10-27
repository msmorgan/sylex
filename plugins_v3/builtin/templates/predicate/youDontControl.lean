/-- "you don't control" -/
template youDontControl : Predicate := {
  spelling := "you don't control",
  body :=
    Semantics.Templates.Predicate.not (Semantics.Templates.Predicate.controlledBy Semantics.Templates.NounPhrase.you)
}
