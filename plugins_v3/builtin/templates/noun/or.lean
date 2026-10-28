template or (phrases : List NounPhrase := []) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := coord .or phrases
}
