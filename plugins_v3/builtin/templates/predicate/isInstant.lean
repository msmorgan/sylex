template isInstant : Predicate := {
  spelling := "instant",
  body :=
    .hasType Semantics.CardType.instant
}
