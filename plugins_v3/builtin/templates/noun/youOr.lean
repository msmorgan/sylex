/-- "you or <subject>" -/
template youOr (subject : NounPhrase) : NounPhrase := {
  spelling := "you or <subject>",
  body :=
    eitherOf you subject
}
