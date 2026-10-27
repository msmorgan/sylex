/-- "<who> controls" -/
primitive controlledBy (who : NounPhrase) : Predicate := {
  spelling := "<who> controls",
  body :=
    Semantics.Primitives.Predicate.hasPossessor Semantics.PossessorAxis.controller who
}
