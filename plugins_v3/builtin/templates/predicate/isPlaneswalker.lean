template isPlaneswalker : Predicate := {
  spelling := "planeswalker",
  body :=
    .hasType Semantics.CardType.planeswalker
}
