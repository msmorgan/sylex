/-- "you and <subject>" -/
template youAnd (subject : NounPhrase) : NounPhrase := {
  spelling := "you and <subject>",
  body :=
    Semantics.Templates.NounPhrase.both Semantics.Templates.NounPhrase.you subject
}
