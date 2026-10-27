template thisRoom : NounPhrase := {
  spelling := "this Room",
  body :=
    .headed (.subtype room) .this
}
