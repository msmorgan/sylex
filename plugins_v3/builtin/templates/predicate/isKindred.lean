template isKindred : Predicate := {
  spelling := "kindred",
  body :=
    Semantics.Templates.Predicate.hasType Semantics.CardType.kindred
}
