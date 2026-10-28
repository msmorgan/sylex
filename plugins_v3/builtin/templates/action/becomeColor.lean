/-- "<subject> becomes <colors> [until …]" -/
template becomeColor (subject : NounPhrase) (colors : ColorSpec)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by duration.
  body :=
    establish
      (qualityChange subject .sets (colored colors))
      duration
}
