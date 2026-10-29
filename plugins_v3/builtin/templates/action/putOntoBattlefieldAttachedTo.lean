/-- "put <subject> [from <from_>] onto the battlefield attached to <host>" [CR#303.4i] -/
-- [CR#303.4i]
template putOntoBattlefieldAttachedTo (subject host : NounPhrase)
    (from_ : Location := wherever) : Action := {
  -- TODO(spelling): Omit the origin phrase when from_ is wherever; otherwise print 'from' and that
  -- location.
  body :=
    zoneChange subject battlefield
      from_ [attachedTo host]
}
