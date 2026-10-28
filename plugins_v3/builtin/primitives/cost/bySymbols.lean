primitive bySymbols (change : Delta ManaCostTerm) (coloredOnly : Bool) : CostShift := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CostShift.bySymbols change coloredOnly
}
