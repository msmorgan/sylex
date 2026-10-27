primitive isTapped : Predicate := {
  spelling := "tapped",
  body :=
    Semantics.Primitives.Predicate.hasStatus Semantics.Status.tapped
}
