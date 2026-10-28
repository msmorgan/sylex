/-- "<subject> becomes <colors> [until …]" -/
template becomeColor (subject : NounPhrase) (colors : ColorSpec)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> becomes <colors> [until …]; string substitution cannot express its omitted or inflected parts.
  body :=
    establish
      (qualityChange subject Semantics.QualityOp.sets (colored colors))
      duration
}
