/-- "<subject> phases out" [CR#702.26b] -/
-- [CR#702.26b]
template phaseOut : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> phases out; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.setStatus Semantics.Status.phasedOut Semantics.Templates.NounPhrase.theActor
}
