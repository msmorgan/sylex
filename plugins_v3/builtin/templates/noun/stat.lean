/-- A printed power, toughness, loyalty, or defense. -/
template stat (value : Nat) : Option Amount := {
  -- TODO(spelling): A printed power, toughness, loyalty, or defense.
  body :=
    Option.some (Semantics.Templates.Amount.amountLit ↑value)
}
