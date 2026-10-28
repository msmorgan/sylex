/-- "<subject> enters [from <from_>]" -/
template enter (from_ : Option Location := none) (riders : List MoveRider := []) :
    Action := {
  -- TODO(spelling): Context-dependent wording: <subject> enters [from <from_>]; string substitution cannot express its omitted or inflected parts.
  body :=
    actionMove theActor (Option.getD from_ wherever)
      (some (zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival
      riders
}
