/-- A literal integer amount. -/
primitive lit (value : Int) : Amount := {
  spelling := "<value>",
  body := Semantics.Primitives.Amount.lit value
}
