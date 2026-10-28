/-- The Characteristic Edit one stat's delta writes: raised, lowered, or set. -/
template statEdit (axis : CharacteristicStat) (delta : Delta Amount) :
    CharacteristicEdit := {
  -- TODO(spelling): The Characteristic Edit one stat's delta writes: raised, lowered, or set.
  body :=
    match delta with
    | Semantics.Delta.up amount => characteristicEditStat Semantics.QualityOp.adds axis amount
    | Semantics.Delta.down amount => characteristicEditStat Semantics.QualityOp.loses axis amount
    | Semantics.Delta.set amount => characteristicEditStat Semantics.QualityOp.sets axis amount
}
