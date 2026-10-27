/-- "the last chosen player" -/
template theLastChosenPlayer : Predicate := {
  spelling := "the last chosen player",
  body :=
    Semantics.Templates.Predicate.and
      [Semantics.Templates.Predicate.isPlayer, Semantics.Templates.Predicate.predicateChosen Semantics.ChoiceRef.theLatestChoice]
}
