primitive hasSubtype (s : Subtype) : Predicate := {
  spelling := "<s>",
  body :=
    Semantics.Primitives.Predicate.hasSubtype s
}
