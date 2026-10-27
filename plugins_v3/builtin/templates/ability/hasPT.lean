/-- "except it's <power>/<toughness>" -/
template hasPT (power toughness : Amount) : CopyExcept := {
  -- TODO(spelling): Context-dependent wording: except it's <power>/<toughness>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.CopyExcept.copyExceptEdits
      [Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power power,
        Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness toughness]
}
