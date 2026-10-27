/-- "<subject>'s total cost is reduced by <cost>", symbol by symbol [CR#702.48c,118.7]. -/
-- [CR#702.48c,118.7]
template reducedBySymbols (subject : NounPhrase) (cost : ManaCostTerm) : StaticSpec := {
  spelling := "<subject>'s total cost is reduced by <cost>",
  body :=
    Semantics.Templates.StaticSpec.costShift subject (Semantics.Templates.CostShift.costShiftBySymbols (Semantics.Delta.down cost) Bool.false)
}
