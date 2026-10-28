/-- "more than N": the count a prohibition bounds ("can't cast more than one spell"). -/
template moreThan (n : Nat) : Quantity := {
  spelling := "more than <n>",
  body :=
    quantityMoreThanOf (amountLit ↑n)
}
