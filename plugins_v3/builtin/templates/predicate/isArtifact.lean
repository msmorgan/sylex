template isArtifact : Predicate := {
  spelling := "artifact",
  body :=
    hasType Semantics.CardType.artifact
}
