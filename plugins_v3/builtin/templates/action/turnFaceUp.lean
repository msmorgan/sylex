/-- "turn <subject> face up" -/
template turnFaceUp (subject : NounPhrase) : Action := {
  spelling := "turn <subject> face up",
  body := setStatus faceUp subject
}
