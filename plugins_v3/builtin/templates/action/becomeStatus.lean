/-- "<subject> becomes <status>" [CR#603.2e] -/
-- [CR#603.2e]
template becomeStatus (status : Status) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> becomes <status>; string substitution cannot express its omitted or inflected parts.
  body :=
    setStatus status theActor
}
