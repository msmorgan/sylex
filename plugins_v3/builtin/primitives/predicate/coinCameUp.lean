/-- Authoring primitive for `Predicate.coinCameUp`. -/
primitive coinCameUp (face : CoinFace) : Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.coinCameUp face
}
