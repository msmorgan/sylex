/-- "each opponent" -/
template eachOpponent : NounPhrase := {
  spelling := "each opponent",
  body :=
    Semantics.Templates.NounPhrase.each Semantics.Templates.Predicate.isOpponent
}
