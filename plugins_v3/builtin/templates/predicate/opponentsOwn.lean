/-- "your opponents own" -/
template opponentsOwn : Predicate := {
  spelling := "your opponents own",
  body :=
    ownedBy yourOpponents
}
