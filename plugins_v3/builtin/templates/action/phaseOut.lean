/-- "<subject> phases out" [CR#702.26b] -/
-- [CR#702.26b]
template phaseOut : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> phases out; string substitution cannot express its omitted or inflected parts.
  body :=
    setStatus Semantics.Status.phasedOut theActor
}
