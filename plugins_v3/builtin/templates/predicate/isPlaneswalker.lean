template isPlaneswalker : Predicate := {
  spelling := "planeswalker",
  body :=
    Semantics.Templates.Predicate.hasType Semantics.CardType.planeswalker
}
