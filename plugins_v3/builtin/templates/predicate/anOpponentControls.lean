/-- "an opponent controls" -/
template anOpponentControls : Predicate := {
  spelling := "an opponent controls",
  body :=
    Semantics.Templates.Predicate.controlledBy Semantics.Templates.NounPhrase.anOpponent
}
