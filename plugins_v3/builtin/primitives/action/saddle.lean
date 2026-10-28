/-- "<subject> saddles <permanent>" [CR#702.171a] -/
-- [CR#702.171a]
primitive saddle (permanent : NounPhrase) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> saddles <permanent>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.saddle permanent
}
