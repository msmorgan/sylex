primitive isFaceUp : Predicate := {
  spelling := "face up",
  body :=
    Semantics.Primitives.Predicate.hasStatus Semantics.Status.faceUp
}
