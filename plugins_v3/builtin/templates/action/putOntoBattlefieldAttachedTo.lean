/-- "put <subject> [from <from_>] onto the battlefield attached to <host>" [CR#303.4i] -/
-- [CR#303.4i]
template putOntoBattlefieldAttachedTo (subject host : NounPhrase)
    (from_ : Option Location := none) : Action := {
  -- TODO(spelling): Context-dependent wording: put <subject> [from <from_>] onto the battlefield attached to <host>; string substitution cannot express its omitted or inflected parts.
  body :=
    zoneChange subject battlefield
      (Option.getD from_ wherever) [attachedTo host]
}
