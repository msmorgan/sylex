primitive chooseNewTargets (subject : NounPhrase) : Action := {
  spelling := "choose new targets for <subject>",
  body :=
    Semantics.Primitives.Action.chooseNewTargets subject
}
