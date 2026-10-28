/-- "Paired", "paired with <counterpart>", "encoded on <counterpart>": the member stands in the
Relation `label`. [CR#702.95b,702.99b] -/
primitive related (label : RelationLabel) (counterpart : Option NounPhrase := none) :
    Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.related label counterpart
}
