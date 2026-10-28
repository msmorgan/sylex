/-- "an opponent controls" -/
template anOpponentControls : Predicate := {
  spelling := "an opponent controls",
  body :=
    controlledBy anOpponent
}
