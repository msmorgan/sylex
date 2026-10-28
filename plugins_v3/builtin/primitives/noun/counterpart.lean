/-- "The creature it haunts": the object on the second side of the Relation `label` whose first
side is `subject` [CR#702.55b]. -/
primitive counterpart (label : RelationLabel) (subject : NounPhrase) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.counterpart label subject
}
