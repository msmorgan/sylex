/-- "<subject> becomes <added> in addition to its other types [until …]" -/
template become (subject : NounPhrase) (added : CharacteristicBundle)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> becomes <added> in addition to its other types [until …]; string substitution cannot express its omitted or inflected parts.
  body :=
    establish
      (qualityChange subject Semantics.QualityOp.adds
        (bundle added none))
      duration
}
