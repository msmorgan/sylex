/-- Several static clauses sharing one subject, as one action. -/
template establishFor (subject : NounPhrase) (parts : List StaticSpec)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by duration. Format and coordinate the
  -- parts list entries, including the empty case.
  body :=
    establish (conjunction parts (some subject)) duration
}
