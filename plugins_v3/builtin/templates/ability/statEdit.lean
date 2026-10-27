/-- The Characteristic Edit one stat's delta writes: raised, lowered, or set. -/
template statEdit (axis : CharacteristicStat) (delta : Delta Amount) :
    CharacteristicEdit := {
  -- TODO(spelling): The Characteristic Edit one stat's delta writes: raised, lowered, or set.
  body :=
    match delta with
    | Semantics.Delta.up amount => Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.adds axis amount
    | Semantics.Delta.down amount => Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.loses axis amount
    | Semantics.Delta.set amount => Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets axis amount
}
