/-- Authoring primitive for `Predicate.coinCameUp`. -/
primitive coinCameUp (face : CoinFace) : Predicate := {
  body :=
    Semantics.Primitives.Predicate.coinCameUp face
}
