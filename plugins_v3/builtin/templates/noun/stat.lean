/-- A printed power, toughness, loyalty, or defense. -/
template stat (value : Nat) : Option Amount := {
  body :=
    some (lit ↑value)
}
