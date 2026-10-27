primitive costShiftBySymbols (change : Semantics.Delta Semantics.ManaCostTerm) (coloredOnly : Bool) : Semantics.CostShift := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CostShift.bySymbols change coloredOnly
}
