/-- "put <subject> <destination>" -/
template put (subject : NounPhrase) (destination : Location)
    (riders : List MoveRider := []) : Action := {
  -- TODO(spelling): put <subject> <destination>; wording varies with riders, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    zoneChange subject destination wherever riders
}
