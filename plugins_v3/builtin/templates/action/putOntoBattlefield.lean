/-- "put <subject> [from <from_>] onto the battlefield" -/
template putOntoBattlefield (subject : NounPhrase) (from_ : Option Location := none)
    (riders : List MoveRider := []) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by from_. Format and coordinate the riders
  -- list entries, including the empty case.
  body :=
    zoneChange subject battlefield
      (Option.getD from_ wherever) riders
}
