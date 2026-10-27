/-- "<who> owns" -/
primitive ownedBy (who : NounPhrase) : Predicate := {
  spelling := "<who> owns",
  body :=
    Semantics.Primitives.Predicate.hasPossessor Semantics.PossessorAxis.owner who
}
