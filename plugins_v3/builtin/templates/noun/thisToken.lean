template thisToken : NounPhrase := {
  spelling := "this token",
  body :=
    Semantics.Templates.NounPhrase.headed Semantics.NounWord.token Semantics.Templates.NounPhrase.this_
}
