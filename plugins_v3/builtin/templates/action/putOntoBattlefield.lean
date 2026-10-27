/-- "put <subject> [from <from_>] onto the battlefield" -/
template putOntoBattlefield (subject : NounPhrase) (from_ : Option Location := none)
    (riders : List MoveRider := []) : Action := {
  -- TODO(spelling): Context-dependent wording: put <subject> [from <from_>] onto the battlefield; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.zoneChange subject Semantics.Templates.Location.battlefield
      (Option.getD from_ Semantics.Templates.Location.wherever) riders
}
