/-- "source": the object dealing the damage under discussion. -/
primitive isSource : Predicate := {
  spelling := "source",
  body :=
    Semantics.Primitives.Predicate.isSource
}
