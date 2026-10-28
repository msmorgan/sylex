/-- "<subject> enters [from <from_>]" -/
template enter (from_ : Option Location := none) (riders : List MoveRider := []) :
    Action := {
  -- TODO(spelling): Include or omit the clauses supplied by from_. Format and coordinate the riders
  -- list entries, including the empty case.
  body :=
    move theActor (Option.getD from_ wherever)
      (some (zone .battlefield none)) .arrival
      riders
}
