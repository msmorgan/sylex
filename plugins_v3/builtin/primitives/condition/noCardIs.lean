/-- "No <scope> in your starting deck <trait>." [CR#702.139a] -/
-- [CR#702.139a]
primitive noCardIs (scope : Predicate) (trait : DeckTrait) : DeckCondition := {
  spelling := "No <scope> in your starting deck <trait>.",
  body :=
    Semantics.Primitives.DeckCondition.noCardIs scope trait
}
