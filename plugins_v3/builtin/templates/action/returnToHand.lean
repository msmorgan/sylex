/-- "return <subject> [from <from_>] to its owner's hand" -/
template returnToHand (subject : NounPhrase) (from_ : Location := wherever) :
    Action := {
  -- TODO(spelling): Omit the origin phrase when from_ is wherever; otherwise print 'from' and that
  -- location.
  body :=
    zoneChange subject hand from_ []
}
