/-- "return <subject> [from <from_>] to the battlefield [riders]" -/
template returnToBattlefield (subject : NounPhrase) (riders : List MoveRider := [])
    (from_ : Option Location := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by from_. Format and coordinate the riders
  -- list entries, including the empty case.
  body :=
    zoneChange subject battlefield
      (Option.getD from_ wherever) riders
}
