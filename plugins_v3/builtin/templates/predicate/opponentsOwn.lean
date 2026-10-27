/-- "your opponents own" -/
template opponentsOwn : Predicate := {
  spelling := "your opponents own",
  body :=
    Semantics.Templates.Predicate.ownedBy Semantics.Templates.NounPhrase.yourOpponents
}
