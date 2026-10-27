/-- "you or <subject>" -/
template youOr (subject : NounPhrase) : NounPhrase := {
  spelling := "you or <subject>",
  body :=
    Semantics.Templates.NounPhrase.eitherOf Semantics.Templates.NounPhrase.you subject
}
