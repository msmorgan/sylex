primitive ownerOf (subject : NounPhrase) : NounPhrase := {
  spelling := "<subject>'s owner",
  body :=
    Semantics.Primitives.NounPhrase.possessorOf Semantics.PossessorAxis.owner subject
}
