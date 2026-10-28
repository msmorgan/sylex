/-- "except it's <power>/<toughness>" -/
template hasPT (power toughness : Amount) : CopyExcept := {
  -- TODO(spelling): Context-dependent wording: except it's <power>/<toughness>; string substitution cannot express its omitted or inflected parts.
  body :=
    copyExceptEdits
      [characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power power,
        characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness toughness]
}
