/-- Authoring primitive for `Predicate.hasSubtype`. -/
primitive hasSubtype (s : Subtype) : Predicate := {
  spelling := "<s>",
  body :=
    Semantics.Primitives.Predicate.hasSubtype s
}
