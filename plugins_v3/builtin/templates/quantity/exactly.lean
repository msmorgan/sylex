template exactly (n : Nat) : Quantity := {
  spelling := "<n>",
  body := range (some n) (some n)
}
