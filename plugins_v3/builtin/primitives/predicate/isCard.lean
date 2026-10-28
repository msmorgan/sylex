/-- Authoring primitive for `Predicate.isCard`. -/
primitive isCard : Predicate := {
  spelling := "card",
  body :=
    Semantics.Primitives.Predicate.isCard
}
