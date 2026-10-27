template isInstant : Predicate := {
  spelling := "instant",
  body :=
    Semantics.Templates.Predicate.hasType Semantics.CardType.instant
}
