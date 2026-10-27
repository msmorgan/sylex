/-- "costs <cost> less", symbol by symbol [CR#702.48c,118.7] -/
-- [CR#702.48c,118.7]
template lessBySymbols (cost : ManaCostTerm) (coloredOnly : Bool := false) : CostShift := {
  -- TODO(spelling): costs <cost> less; wording varies with coloredOnly, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Templates.CostShift.costShiftBySymbols (Semantics.Delta.down cost) coloredOnly
}
