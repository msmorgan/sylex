template isLand : Predicate := {
  spelling := "land",
  body :=
    Semantics.Templates.Predicate.hasType Semantics.CardType.land
}
