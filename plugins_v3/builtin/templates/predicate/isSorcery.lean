template isSorcery : Predicate := {
  spelling := "sorcery",
  body :=
    Semantics.Templates.Predicate.hasType Semantics.CardType.sorcery
}
