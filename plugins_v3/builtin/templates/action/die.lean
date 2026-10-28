/-- "<subject> dies": put into a graveyard from the battlefield [CR#700.4]. -/
-- [CR#700.4]
template die : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> dies; string substitution cannot express its omitted or inflected parts.
  body :=
    actionMove theActor (zone Semantics.Zone.battlefield none)
      (some (zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure []
}
