/-- "opponent": an opponent of you. -/
template isOpponent : Predicate := {
  spelling := "opponent",
  body :=
    isOpponentOf you
}
