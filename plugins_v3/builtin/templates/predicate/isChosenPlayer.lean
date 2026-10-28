/-- "the chosen player" -/
template isChosenPlayer : Predicate := {
  spelling := "the chosen player",
  body :=
    and [isPlayer, isChosen]
}
