/-- "<subject> has base power and toughness P/T" -/
template getsBase (subject : NounPhrase) (power toughness : Amount) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <subject> has base power and toughness P/T; string substitution cannot express its omitted or inflected parts.
  body :=
    getsPt subject (setTo power) (setTo toughness)
}
