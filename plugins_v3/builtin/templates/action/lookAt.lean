template lookAt (cards : NounPhrase) : Action := {
  spelling := "look at <cards>",
  body :=
    expose lookAt (Templates.Exposed.cards cards)
}
