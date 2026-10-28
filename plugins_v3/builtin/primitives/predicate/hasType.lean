/-- Authoring primitive for `Predicate.hasType`. -/
primitive hasType (t : CardType) : Predicate := {
  spelling := "<t>",
  body :=
    Semantics.Primitives.Predicate.hasType t
}
