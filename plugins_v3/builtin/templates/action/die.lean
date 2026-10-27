/-- "<subject> dies": put into a graveyard from the battlefield [CR#700.4]. -/
-- [CR#700.4]
template die : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> dies; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.actionMove Semantics.Templates.NounPhrase.theActor (Semantics.Templates.Location.zone Semantics.Zone.battlefield Option.none)
      (Option.some (Semantics.Templates.Location.zone Semantics.Zone.graveyard Option.none)) Semantics.ObservationPoint.departure []
}
