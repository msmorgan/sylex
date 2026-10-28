/-- "put <subject> [from <from_>] onto the battlefield tapped" -/
template putOntoBattlefieldTapped (subject : NounPhrase)
    (from_ : Option Location := none) : Action := {
  -- TODO(spelling): Context-dependent wording: put <subject> [from <from_>] onto the battlefield tapped; string substitution cannot express its omitted or inflected parts.
  body :=
    zoneChange subject battlefield
      (Option.getD from_ wherever) [entersAs Semantics.Status.tapped]
}
