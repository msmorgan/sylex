/-- "<subject> gets +P/+T [until …]" as an action. -/
template get (subject : NounPhrase) (power toughness : Delta Amount)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> gets +P/+T [until …]; string substitution cannot express its omitted or inflected parts.
  body :=
    establish (getsPt subject power toughness) duration
}
