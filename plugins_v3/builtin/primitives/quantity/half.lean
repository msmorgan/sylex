/-- Half the given amount, rounded in the specified direction. -/
primitive half (rounding : RoundMode) (amount : Amount) : Amount := {
  spelling := "half <amount>, rounded <rounding>",
  body :=
    Semantics.Primitives.Amount.half rounding amount
}
