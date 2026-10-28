/-- "return <subject> to <zone>" -/
template returnTo (subject : NounPhrase) (destination : Location)
    (riders : List MoveRider) : Action := {
  -- TODO(spelling): Context-dependent wording: return <subject> to <zone>; string substitution cannot express its omitted or inflected parts.
  body :=
    zoneChange subject destination wherever riders
}
