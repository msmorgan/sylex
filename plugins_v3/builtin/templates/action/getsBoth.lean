template getsBoth (subject : NounPhrase) (delta : Delta Amount)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    get subject delta delta duration
}
