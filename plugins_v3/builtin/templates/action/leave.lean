/-- "<subject> leaves <from_>", the battlefield unless said. -/
template leave (from_ : Location := battlefield) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> leaves <from_>; string substitution cannot express its omitted or inflected parts.
  body :=
    actionMove theActor from_ none Semantics.ObservationPoint.departure []
}
