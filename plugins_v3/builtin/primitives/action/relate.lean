/-- Make the subject stand in the named relation with its counterpart: haunting, pairing, or
encoding [CR#702.55b,702.95b,702.99b]. -/
primitive relate (subject : NounPhrase) (label : RelationLabel) (counterpart : NounPhrase) :
    Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.relate subject label counterpart
}
