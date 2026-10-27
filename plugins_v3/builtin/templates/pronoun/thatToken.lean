/-- "that token" -/
template thatToken : NounPhrase := {
  spelling := "that token",
  body :=
    Semantics.Templates.NounPhrase.that Semantics.NounWord.token
}
