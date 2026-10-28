/-- "the last chosen player" -/
template theLastChosenPlayer : Predicate := {
  spelling := "the last chosen player",
  body :=
    and
      [isPlayer, predicateChosen Semantics.ChoiceRef.theLatestChoice]
}
