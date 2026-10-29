/-- "put <subject> [from <from_>] onto the battlefield under your control" -/
template putOntoBattlefieldUnderYourControl (subject : NounPhrase)
    (from_ : Location := wherever) : Action := {
  -- TODO(spelling): Omit the origin phrase when from_ is wherever; otherwise print 'from' and that
  -- location.
  body :=
    zoneChange subject battlefield
      from_ [under you]
}
