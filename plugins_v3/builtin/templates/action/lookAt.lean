template lookAt (cards : NounPhrase) : Action := {
  spelling := "look at <cards>",
  body :=
    expose Semantics.ExposeVerb.lookAt (Exposed.cards cards)
}
