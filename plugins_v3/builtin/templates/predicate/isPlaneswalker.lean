/-- "planeswalker": a Predicate selecting planeswalker Objects. -/
template isPlaneswalker : Predicate := {
  spelling := "planeswalker",
  body :=
    hasType Semantics.CardType.planeswalker
}
