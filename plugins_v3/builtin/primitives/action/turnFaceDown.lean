primitive turnFaceDown (subject : NounPhrase) (spec : FaceDown) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.turnFaceDown subject spec
}
