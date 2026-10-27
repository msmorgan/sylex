/-- "<subject> gains haste [until …]" -/
template gainHaste (subject : NounPhrase) (duration : Option Duration := none) : Action := {
  body :=
    gain subject haste duration
}
