/-- "unattach <subject>" -/
primitive unattach (subject : NounPhrase) : Action := {
  spelling := "unattach <subject>",
  body :=
    Semantics.Primitives.Action.attachment Semantics.AttachMove.unattached subject Option.none
}
