template lookAt (cards : NounPhrase) : Action := {
  spelling := "look at <cards>",
  body :=
    Semantics.Templates.Action.expose Semantics.ExposeVerb.lookAt (Semantics.Templates.Exposed.cards cards)
}
