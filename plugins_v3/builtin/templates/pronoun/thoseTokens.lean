/-- "those tokens" -/
template thoseTokens : NounPhrase := {
  spelling := "those tokens",
  body :=
    Semantics.Templates.NounPhrase.those Semantics.NounWord.token
}
