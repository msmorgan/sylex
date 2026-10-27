/-- "non<p>", as "nonland" or "noncreature". -/
primitive non (p : Predicate) : Predicate := {
  spelling := "non<p>",
  body :=
    Semantics.Primitives.Predicate.not p
}
