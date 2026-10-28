/-- "put <subject> [from <from_>] onto the battlefield attached to <host>" [CR#303.4i] -/
-- [CR#303.4i]
template putOntoBattlefieldAttachedTo (subject host : NounPhrase)
    (from_ : Option Location := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by from_.
  body :=
    zoneChange subject battlefield
      (Option.getD from_ wherever) [attachedTo host]
}
