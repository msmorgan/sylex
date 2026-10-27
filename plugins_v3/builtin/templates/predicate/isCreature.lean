template isCreature : Predicate := {
  spelling := "creature",
  body :=
    Semantics.Templates.Predicate.hasType Semantics.CardType.creature
}
