template orMore (n : Nat) : Quantity := {
  spelling := "<n> or more",
  body := range (some n) none
}
