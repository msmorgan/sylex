/-- "the chosen player" -/
template isChosenPlayer : Predicate := {
  spelling := "the chosen player",
  body :=
    Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isPlayer, Semantics.Templates.Predicate.isChosen]
}
