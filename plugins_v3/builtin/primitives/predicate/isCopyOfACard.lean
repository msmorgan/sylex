/-- Authoring primitive for `Predicate.isCopyOfACard`. -/
primitive isCopyOfACard : Predicate := {
  spelling := "copy of a card",
  body :=
    Semantics.Primitives.Predicate.isCopyOfACard
}
