template isBattle : Predicate := {
  spelling := "battle",
  body :=
    Semantics.Templates.Predicate.hasType Semantics.CardType.battle
}
