/-- "put <subject> into <destination> [from <from_>] [<riders>]", as "in any order" or "under
its owner's control". -/
template putInto (subject : NounPhrase) (destination : Location)
    (from_ : Option Location := none) (riders : List MoveRider := []) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by from_. Format and coordinate the riders
  -- list entries, including the empty case.
  body :=
    zoneChange subject destination (Option.getD from_ wherever) riders
}
