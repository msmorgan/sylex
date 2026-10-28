/-- Authoring primitive for `Predicate.isToken`. -/
primitive isToken : Predicate := {
  spelling := "token",
  body :=
    Semantics.Primitives.Predicate.isToken
}
