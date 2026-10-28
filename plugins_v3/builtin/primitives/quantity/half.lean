/-- Half the given amount, rounded in the specified direction. -/
primitive half (rounding : RoundMode) (amount : Amount) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.half rounding amount
}
