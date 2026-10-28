/-- "<subject> crews <vehicle>" [CR#702.122a] -/
-- [CR#702.122a]
primitive crew (vehicle : NounPhrase) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> crews <vehicle>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.crew vehicle
}
