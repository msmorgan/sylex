/-- "put <subject> [from <from_>] onto the battlefield under your control" -/
template putOntoBattlefieldUnderYourControl (subject : NounPhrase)
    (from_ : Option Location := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by from_.
  body :=
    zoneChange subject battlefield
      (Option.getD from_ wherever) [under you]
}
