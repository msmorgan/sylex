/-- A conjunction of Noun Phrases. -/
template and (phrases : List NounPhrase := []) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := coord .and phrases
}
