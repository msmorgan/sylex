template isArtifact : Predicate := {
  spelling := "artifact",
  body :=
    Semantics.Templates.Predicate.hasType Semantics.CardType.artifact
}
