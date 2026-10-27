/-- "<subject> gets +P/+T [until …]": one change to both stats, the subject written once. -/
template getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <subject> gets +P/+T [until …]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.StaticSpec.characteristicChange subject
      [Semantics.Templates.CharacteristicEdit.statEdit Semantics.CharacteristicStat.power power,
        Semantics.Templates.CharacteristicEdit.statEdit Semantics.CharacteristicStat.toughness toughness]
}
