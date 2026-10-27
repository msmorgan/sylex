/-- "except by <objects>": the objects a prohibition exempts [CR#702.9b,702.111b]. -/
-- [CR#702.9b,702.111b]
primitive byObjects (objects : NounPhrase) : DeonticExcept := {
  spelling := "except by <objects>",
  body :=
    Semantics.Primitives.DeonticExcept.byObjects objects
}
