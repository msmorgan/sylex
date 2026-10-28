/-- Authoring primitive for `Predicate.withMostVotes`. -/
primitive withMostVotes : Predicate := {
  spelling := "with the most votes",
  body :=
    Semantics.Primitives.Predicate.withMostVotes
}
