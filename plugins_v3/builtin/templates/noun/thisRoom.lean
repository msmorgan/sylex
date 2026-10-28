template thisRoom : NounPhrase := {
  spelling := "this Room",
  body :=
    headed (Semantics.NounWord.subtype room) this_
}
