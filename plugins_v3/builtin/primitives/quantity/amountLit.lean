primitive amountLit (value : Int) : Semantics.Amount := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Amount.lit value
}
