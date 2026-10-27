template isCreature : Predicate := {
  spelling := "creature",
  body :=
    .hasType Semantics.CardType.creature
}
