/-- "the number of <subject>" -/
primitive numberOf (subject : NounPhrase) : Amount := {
  spelling := "the number of <subject>",
  body :=
    Semantics.Primitives.Amount.countOf subject
}
