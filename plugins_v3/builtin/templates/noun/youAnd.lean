/-- "you and <subject>" -/
template youAnd (subject : NounPhrase) : NounPhrase := {
  spelling := "you and <subject>",
  body :=
    both you subject
}
