/-- Authoring primitive for `Predicate.hasSupertype`. -/
primitive hasSupertype (s : Supertype) : Predicate := {
  spelling := "<s>",
  body :=
    Semantics.Primitives.Predicate.hasSupertype s
}
