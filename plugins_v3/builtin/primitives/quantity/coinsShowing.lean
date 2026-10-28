/-- The number of coins showing the specified face. -/
primitive coinsShowing (face : CoinFace) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.coinsShowing face
}
