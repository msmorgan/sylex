/-- "turn <subject> face up" -/
primitive turnFaceUp (subject : NounPhrase) : Action := {
  spelling := "turn <subject> face up",
  body :=
    Semantics.Primitives.Action.setStatus Semantics.Status.faceUp subject
}
