primitive controllerOf (subject : NounPhrase) : NounPhrase := {
  spelling := "<subject>'s controller",
  body :=
    Semantics.Primitives.NounPhrase.possessorOf Semantics.PossessorAxis.controller subject
}
