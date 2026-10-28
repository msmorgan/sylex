/-- "put <subject> [from <from_>] onto the battlefield tapped" -/
template putOntoBattlefieldTapped (subject : NounPhrase)
    (from_ : Option Location := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by from_.
  body :=
    zoneChange subject battlefield
      (Option.getD from_ wherever) [entersAs .tapped]
}
