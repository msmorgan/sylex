primitive bySymbols (change : Delta ManaCostTerm) (coloredOnly : Bool) : CostShift := {
  body := Semantics.Primitives.CostShift.bySymbols change coloredOnly
}
