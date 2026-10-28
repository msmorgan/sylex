/-- "target …" -/
primitive target (quantity : Quantity) : DetPhrase := {
  spelling := "<quantity> target",
  body := Semantics.Primitives.DetPhrase.target quantity
}
