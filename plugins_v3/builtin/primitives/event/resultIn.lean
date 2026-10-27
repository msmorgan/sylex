/-- "a result in <quantity>" -/
primitive resultIn (quantity : Quantity) : RollWatch := {
  spelling := "a result in <quantity>",
  body :=
    Semantics.Primitives.RollWatch.resultIn quantity
}
