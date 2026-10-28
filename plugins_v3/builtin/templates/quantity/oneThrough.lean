template oneThrough (n : Nat) : Quantity := {
  spelling := "one to <n>",
  body := range (some 1) (some n)
}
