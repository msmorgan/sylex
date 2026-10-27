/-- "a number <quantity>" -/
primitive number (quantity : Quantity) : ChoiceDomain := {
  spelling := "a number <quantity>",
  body :=
    Semantics.Primitives.ChoiceDomain.number quantity
}
