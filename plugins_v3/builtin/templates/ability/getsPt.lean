/-- "<subject> gets +P/+T [until …]": one change to both stats, the subject written once. -/
template getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <subject> gets +P/+T [until …]; string substitution cannot express its omitted or inflected parts.
  body :=
    characteristicChange subject
      [statEdit .power power,
        statEdit .toughness toughness]
}
