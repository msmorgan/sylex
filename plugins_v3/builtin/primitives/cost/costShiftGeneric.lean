primitive costShiftGeneric (change : Semantics.Delta Semantics.Amount) (floor : Option Semantics.Amount) : Semantics.CostShift := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CostShift.generic change floor
}
