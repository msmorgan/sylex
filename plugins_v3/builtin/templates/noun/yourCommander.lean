/-- "your commander" -/
template yourCommander : NounPhrase := {
  spelling := "your commander",
  body :=
    Semantics.Templates.NounPhrase.designated Semantics.Definitions.commander Semantics.Templates.NounPhrase.you
}
