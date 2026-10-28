/-- Turn the subject face down with the specified characteristics [CR#701.40a,708.2]. -/
primitive turnFaceDown (subject : NounPhrase) (spec : FaceDown) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.turnFaceDown subject spec
}
