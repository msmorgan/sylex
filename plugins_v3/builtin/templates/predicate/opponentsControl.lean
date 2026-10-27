/-- "your opponents control" -/
template opponentsControl : Predicate := {
  spelling := "your opponents control",
  body :=
    Semantics.Templates.Predicate.controlledBy Semantics.Templates.NounPhrase.yourOpponents
}
