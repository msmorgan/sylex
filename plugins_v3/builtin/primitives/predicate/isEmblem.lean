/-- Authoring primitive for `Predicate.isEmblem`. -/
primitive isEmblem : Predicate := {
  spelling := "emblem",
  body :=
    Semantics.Primitives.Predicate.isEmblem
}
