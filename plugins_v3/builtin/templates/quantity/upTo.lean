template upTo (n : Nat) : Quantity := {
  spelling := "up to <n>",
  body := range none (some n)
}
