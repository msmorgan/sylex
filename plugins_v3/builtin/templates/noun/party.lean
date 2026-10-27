/-- "your party" -/
template party : NounPhrase := {
  spelling := "your party",
  body :=
    Semantics.Templates.NounPhrase.partyOf Semantics.Templates.NounPhrase.you
}
