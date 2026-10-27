/-- "<subject> enters [from <from_>]" -/
template enter (from_ : Option Location := none) (riders : List MoveRider := []) :
    Action := {
  -- TODO(spelling): Context-dependent wording: <subject> enters [from <from_>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.actionMove Semantics.Templates.NounPhrase.theActor (Option.getD from_ Semantics.Templates.Location.wherever)
      (Option.some (Semantics.Templates.Location.zone Semantics.Zone.battlefield Option.none)) Semantics.ObservationPoint.arrival
      riders
}
