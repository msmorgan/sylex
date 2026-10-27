/-- "<subject> gains haste [until …]" -/
template gainHaste (subject : NounPhrase) (duration : Option Duration := none) : Action := {
  -- TODO(spelling): <subject> gains haste [until <duration>]; the optional duration requires conditional punctuation.
  body :=
    gain subject haste duration
}
