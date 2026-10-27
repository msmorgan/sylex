/-- "<subject> phases in" [CR#702.26c] -/
-- [CR#702.26c]
template phaseIn : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> phases in; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.setStatus Semantics.Status.phasedIn Semantics.Templates.NounPhrase.theActor
}
