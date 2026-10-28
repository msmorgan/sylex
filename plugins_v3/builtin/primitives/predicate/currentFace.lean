/-- Authoring primitive for `Predicate.currentFace`. -/
primitive currentFace (side : CardFaceSide) : Predicate := {
  body :=
    Semantics.Primitives.Predicate.currentFace side
}
