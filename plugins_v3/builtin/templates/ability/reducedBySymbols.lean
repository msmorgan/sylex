/-- "<subject>'s total cost is reduced by <cost>", symbol by symbol [CR#702.48c,118.7]. -/
-- [CR#702.48c,118.7]
template reducedBySymbols (subject : NounPhrase) (cost : ManaCostTerm) : StaticSpec := {
  spelling := "<subject>'s total cost is reduced by <cost>",
  body :=
    costShift subject (bySymbols (.down cost) false)
}
