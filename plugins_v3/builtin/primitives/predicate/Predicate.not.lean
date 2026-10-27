/-- "not <p>" -/
primitive Predicate.not (p : Predicate) : Predicate := {
  spelling := "not <p>",
  body :=
    Semantics.Primitives.Predicate.not p
}
