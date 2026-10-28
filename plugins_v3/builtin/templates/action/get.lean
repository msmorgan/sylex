/-- "<subject> gets +P/+T [until …]" as an action. -/
template get (subject : NounPhrase) (power toughness : Delta Amount)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by duration.
  body :=
    establish (getsPt subject power toughness) duration
}
