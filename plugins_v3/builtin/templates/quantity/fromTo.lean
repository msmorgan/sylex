/-- "between N and M" -/
template fromTo (low high : Nat) : Quantity := {
  spelling := "between <low> and <high>",
  body := range (some low) (some high)
}
