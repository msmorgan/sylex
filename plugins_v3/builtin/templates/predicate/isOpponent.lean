/-- "opponent": an opponent of you. -/
template isOpponent : Predicate := {
  spelling := "opponent",
  body :=
    Semantics.Templates.Predicate.isOpponentOf Semantics.Templates.NounPhrase.you
}
