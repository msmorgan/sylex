/-- "between N and M" -/
primitive fromTo (low high : Nat) : Quantity := {
  spelling := "between <low> and <high>",
  body :=
    Semantics.Primitives.Quantity.range (Option.some low) (Option.some high)
}
