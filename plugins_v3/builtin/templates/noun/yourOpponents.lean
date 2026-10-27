/-- "your opponents" -/
template yourOpponents : NounPhrase := {
  spelling := "your opponents",
  body :=
    Semantics.Templates.NounPhrase.allOf Semantics.Templates.Predicate.isOpponent
}
