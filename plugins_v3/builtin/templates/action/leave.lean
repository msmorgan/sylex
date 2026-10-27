/-- "<subject> leaves <from_>", the battlefield unless said. -/
template leave (from_ : Location := battlefield) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> leaves <from_>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.actionMove Semantics.Templates.NounPhrase.theActor from_ Option.none Semantics.ObservationPoint.departure []
}
