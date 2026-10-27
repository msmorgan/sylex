primitive isUntapped : Predicate := {
  spelling := "untapped",
  body :=
    Semantics.Primitives.Predicate.hasStatus Semantics.Status.untapped
}
