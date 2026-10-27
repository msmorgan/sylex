template thisArtifact : NounPhrase := {
  spelling := "this artifact",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.type Semantics.CardType.artifact) Semantics.Templates.NounPhrase.this_
}
