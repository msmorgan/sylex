primitive isFaceDown : Predicate := {
  spelling := "face down",
  body :=
    Semantics.Primitives.Predicate.hasStatus Semantics.Status.faceDown
}
